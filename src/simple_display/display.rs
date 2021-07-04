//! main display module
use rppal::{
    gpio::{Gpio, OutputPin},
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::{thread::sleep, time::Duration};

use crate::simple_display::command::Command;

const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 128;
const BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 2;

/// Represents the SSD1351 Display.
///
/// Use this struct to initialize the driver.
pub struct Ssd1351 {
    spi: Spi,
    rc: OutputPin,
    rst: OutputPin,
    buffer: [u8; BUFFER_SIZE],
}

impl Ssd1351 {
    /// Creates the SSD1351 Display.
    ///
    /// Make sure to reset and initialize the display before use!
    pub fn new(spi: Spi, rc: OutputPin, rst: OutputPin) -> Self {
        Self {
            spi: spi,
            rc: rc,
            rst: rst,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Resets the display.
    pub fn reset(&mut self) -> Result<(), rppal::gpio::Error> {
        let sleep_duration = Duration::from_millis(100);
        self.rst.set_low();
        sleep(sleep_duration);

        self.rst.set_high();
        sleep(sleep_duration);

        Ok(())
    }

    /// Copy an array of bytes to the buffer.
    pub fn update_buffer(&mut self, bytes: &[u8]) {
        let coef_big;
        let coef_little;
        if cfg!(target_endian = "big") {
            coef_little = 0;
            coef_big = 1;
        } else {
            coef_little = 1;
            coef_big = 0;
        }

        for i in (0..BUFFER_SIZE - 1).step_by(2) {
            self.buffer[i] = bytes[i + coef_little];
            self.buffer[i + 1] = bytes[i + coef_big];
        }
    }

        /// Allows to send custom commands to the display.
        pub fn send_command(&mut self, command: Command) -> Result<(), rppal::spi::Error> {
            command.send_command(&mut self.rc, &mut self.spi)?;
            Ok(())
        }
    
        /// Allows to send the date to the display.
        pub fn send_data(&mut self, data: &[u8]) -> Result<(), rppal::spi::Error> {
            Command::send_data(&mut self.spi, data)?;
            Ok(())
        }

    /// Flushes the display, and makes the output visible on the screen.
    pub fn flush(&mut self) -> Result<(), rppal::spi::Error> {
        self.send_command(Command::ColumnAddress)?;
        self.send_command(Command::RowAddress)?;
        self.send_command(Command::DisplayStartLine)?;
        self.send_command(Command::WriteRam)?;

             let buffer = self.buffer;
        self.send_data(&buffer)?;
        Ok(())
    }

        /// Initializes the display.
        pub fn turn_on(&mut self) -> Result<(), rppal::spi::Error> {
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
        pub fn turn_off(&mut self) -> Result<(), rppal::spi::Error> {
            self.send_command(Command::DisplayOff)?;
            Ok(())
        }

    /// Clear the display
    pub fn clear(&mut self) -> Result<(), rppal::gpio::Error> {
        for i in 0..128 * 128 * 2 {
            self.buffer[i] = 0;
        }
        Ok(())
    }
}
