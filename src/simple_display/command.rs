//! Contains all the commands that can be sent to the display

use rppal::{gpio::OutputPin, spi::Spi};

/// Holds commands which can be sent to the display.
pub enum Command {
    Unlock0x12,
    Unlock0xB1,
    DisplayOff,
    ClockDivider,
    Muxratio,
    ColumnAddress,
    RowAddress,
    SegmentRemapping,
    DisplayStartLine,
    DisplayOffset,
    SetGpio,
    SelectInternalDiodeDrop,
    Precharge,
    SetSegmentLowVoltage,
    SetVcomHVoltage,
    Invert,
    ContrastMaster,
    Precharge2,
    Contrast,
    DisplayOn,
    NomalDisplay,
    WriteRam,
}

impl Command {
    /// Sends the command tot he SPI device.
    pub fn send_command(self, dc: &mut OutputPin, spi: &mut Spi) -> Result<(), rppal::spi::Error> {
        let (command, data, len) = match self {
            Self::Unlock0x12 => (0xFD, [0x12, 0, 0], 1),
            Self::Unlock0xB1 => (0xFD, [0xB1, 0, 0], 1),
            Self::DisplayOff => (0xAE, [0, 0, 0], 0),
            Self::ClockDivider => (0xB3, [0xF1, 0, 0], 1),
            Self::Muxratio => (0xCA, [0x7F, 0, 0], 1),
            Self::ColumnAddress => (0x15, [0x00, 0x7F, 0], 2),
            Self::RowAddress => (0x75, [0x00, 0x7F, 0], 2),
            Self::SegmentRemapping => (0xA0, [0x74, 0, 0], 1),
            Self::DisplayStartLine => (0xA1, [0x00, 0, 0], 1),
            Self::DisplayOffset => (0xA2, [0x00, 0, 0], 1),
            Self::SetGpio => (0xB5, [0x00, 0, 0], 1),
            Self::SelectInternalDiodeDrop => (0xAB, [0x01, 0, 0], 1),
            Self::Precharge => (0xB1, [0x32, 0, 0], 1),
            Self::SetSegmentLowVoltage => (0xB4, [0xA0, 0xB5, 0x55], 3),
            Self::SetVcomHVoltage => (0xBE, [0x05, 0, 0], 1),
            Self::Invert => (0xA7, [0x00, 0, 0], 0),
            Self::ContrastMaster => (0xC7, [0x0F, 0, 0], 1),
            Self::Precharge2 => (0xB6, [0x01, 0, 0], 1),
            Self::Contrast => (0xC1, [0xFF, 0xFF, 0xFF], 3),
            Self::DisplayOn => (0xAF, [0, 0, 0], 0),
            Self::NomalDisplay => (0xA6, [0, 0, 0], 0),
            Self::WriteRam => (0x5C, [0, 0, 0], 0),
        };

        dc.set_low();
        spi.write(&[command])?;
        dc.set_high();

        if len > 0 {
            spi.write(&data[0..len])?;
        }
        Ok(())
    }

    /// Write the data to the spi device.
    pub fn send_data(spi: &mut Spi, data: &[u8]) -> Result<(), rppal::spi::Error> {
        if data.len() > 0 {
            let multiplier = 128 * 8 * 2;
            for i in 0..16 {
                spi.write(&data[i * multiplier..(i + 1) * multiplier])?;
            }
        }
        Ok(())
    }
}
