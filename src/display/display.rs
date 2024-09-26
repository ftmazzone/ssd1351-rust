//! main display module
use crate::display::command::Command;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU16},
        Rgb565,
    },
    Pixel,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 128;
const BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 2;

/// Represents the SSD1351 Display.
///
/// Use this struct to initialize the driver.
pub struct Ssd1351<DI> {
    display: DI,
    buffer: [u8; BUFFER_SIZE],
}


impl<DI: WriteOnlyDataCommand> Ssd1351<DI> {
    /// Creates the SSD1351 Display.
    ///
    /// Make sure to reset and initialize the display before use!
    pub fn new(display: DI) -> Self {
        Self {
            display,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Resets the display.
    pub fn reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), DisplayError>
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        // rst.set_high().map_err(|_| DisplayError::BusWriteError)?;
        // delay.delay_ms(100);

        rst.set_low().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(100);

        rst.set_high().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(100);

        Ok(())
    }

    /// Initializes the display.
    pub fn turn_on(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::Unlock0x12)?;
        self.send_command(Command::Unlock0xB1)?;
        self.send_command(Command::DisplayOff)?;
        self.send_command(Command::ClockDivider)?;
        self.send_command(Command::Muxratio)?;
        self.send_command(Command::ColumnAddress)?;
        self.send_command(Command::RowAddress)?;
        self.send_command(Command::SegmentRemapping)?;
        self.send_command(Command::DisplayStartLine)?;
        self.send_command(Command::DisplayOffset)?;
        self.send_command(Command::SetGpio)?;
        self.send_command(Command::SelectInternalDiodeDrop)?;
        self.send_command(Command::Precharge)?;
        self.send_command(Command::SetSegmentLowVoltage)?;
        self.send_command(Command::SetVcomHVoltage)?;
        //self.send_command(Command::Invert)?;
        self.send_command(Command::ContrastMaster)?;
        self.send_command(Command::Precharge2)?;
        self.send_command(Command::Contrast)?;
        self.send_command(Command::DisplayOn)?;
        self.send_command(Command::NomalDisplay)?;
        self.send_command(Command::DisplayOffset)?;
        Ok(())
    }

    /// Turns off the display.
    pub fn turn_off(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOff)?;
        Ok(())
    }

    /// Allows to send custom commands to the display.
    pub fn send_command(&mut self, command: Command) -> Result<(), DisplayError> {
        command.send(&mut self.display)
    }

    /// Flushes the display, and makes the output visible on the screen.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::ColumnAddress)?;
        self.send_command(Command::RowAddress)?;
        self.send_command(Command::DisplayStartLine)?;
        self.send_command(Command::WriteRam)?;

        // Number of bytes 128*128*2;
        let multiplier = 128 * 8 * 2;
        for i in 0..16 {
            self.display
                .send_data(U8(&self.buffer[i * multiplier..(i + 1) * multiplier]));
        }
        self.display.send_data(U8(&self.buffer))
    }
}

impl<DI> DrawTarget for Ssd1351<DI>
where
    DI: WriteOnlyDataCommand,
{
    type Color = Rgb565;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels.into_iter().for_each(|Pixel(point, colour_pixel)| {
            let colour_u16 = RawU16::from(colour_pixel).into_inner();
            let colour = [(colour_u16 >> 8) as u8, colour_u16 as u8];
            let idx = (point.x + point.y * 128) * 2;
            // ignore out-of-bounds drawing
            if idx < 0 || idx >= self.buffer.len() as i32 {
                return;
            }
            let idx = idx as usize;
            self.buffer[idx] = colour[0];
            self.buffer[idx + 1] = colour[1];
        });

        Ok(())
    }

    fn clear(&mut self, fill: Rgb565) -> Result<(), Self::Error> {
        let colour_u16 = RawU16::from(fill).into_inner();
        let colour = [(colour_u16 >> 8) as u8, colour_u16 as u8];
        for x in 0..128 {
            for y in 0..128 {
                let idx = ((x + y * 128) * 2) as usize;
                self.buffer[idx] = colour[0];
                self.buffer[idx + 1] = colour[1];
            }
        }
        Ok(())
    }
}

impl<DI> OriginDimensions for Ssd1351<DI>
where
    DI: WriteOnlyDataCommand,
{
    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}
