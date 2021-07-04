//! Contains all the commands that can be sent to the display

//use super::interface::DisplayInterface;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

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
    WriteRam, // /// Turn display off (0xAE)
              // DisplayOff,
              // /// Turn display on (0xAF)
              // DisplayOn,
              // /// Set up column start and end address (0x15)
              // ColumnAddress {
              //     /// The start column address
              //     start: u8,
              //     /// The end column address
              //     end: u8,
              // },
              // /// Set up row start and end address (0x75)
              // RowAddress {
              //     /// The start row address
              //     start: u8,
              //     /// The end row address
              //     end: u8,
              // },
              // /// Contrast Control (0x81)
              // Contrast(u8),
              // /// Re-map setting in Graphic Display Data RAM  (0xA0)
              // SetRemap(u8),
              // /// Display Start Line (0xA1)
              // StartLine(u8),
              // /// Display Offset (0xA2)
              // Offset(u8),
              // /// Normal Display Mode (0xA4)
              // DisplayModeNormal,
              // /// Multiplex Ratio (0xA8)
              // MuxRatio(u8),
              // /// Phase Length (0xB1)
              // PhaseLength(u8),
              // /// Front Clock Divider / Oscillator Frequency (0xB3)
              // FrontClockDivider(u8),
              // /// Function Selection A (0xAB)
              // FunctionSelectionA(u8),
              // /// Second Pre-Charge Period (0xB6)
              // SecondPreChargePeriod(u8),
              // /// COM deselect voltage level (0xBE)
              // ComVoltageLevel(u8),
              // /// Pre-Charge Voltage (0xBC)
              // PreChargeVoltage(u8),
              // /// Function Selection B (0xD5)
              // FunctionSelectionB(u8),
              // /// Function Selection B (0xD5)
              // CommandLock(u8)
}

impl Command {
    pub(crate) fn send<DI>(self, display: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
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
            // Self::DisplayOn => ([0xAF, 0, 0], 1),
            // Self::DisplayOff => ([0xAE, 0, 0], 1),
            // Self::ColumnAddress { start, end } => ([0x15, start, end], 3),
            // Self::RowAddress { start, end } => ([0x75, start, end], 3),
            // Self::Contrast(value) => ([0x81, value, 0], 2),
            // Self::SetRemap(value) => ([0xA0, value, 0], 2),
            // Self::StartLine(value) => ([0xA1, value, 0], 2),
            // Self::Offset(value) => ([0xA2, value, 0], 2),
            // Self::DisplayModeNormal => ([0xA4, 0, 0], 1),
            // Self::MuxRatio(value) => ([0xA8, value, 0], 2),
            // Self::PhaseLength(value) => ([0xB1, value, 0], 2),
            // Self::FrontClockDivider(value) => ([0xB3, value, 0], 2),
            // Self::FunctionSelectionA(value) => ([0xAB, value, 0], 2),
            // Self::SecondPreChargePeriod(value) => ([0xB6, value, 0], 2),
            // Self::ComVoltageLevel(value) => ([0xBE, value, 0], 2),
            // Self::PreChargeVoltage(value) => ([0xBC, value, 0], 2),
            // Self::FunctionSelectionB(value) => ([0xD5, value, 0], 2),
            // Self::CommandLock(value) => ([0xFD, value, 0], 2),
        };
        //display.send_commands(U8(&data[0..len]))
        // Send command over the interface
        display.send_commands(U8(&[command]))?;

        if len > 0 {
            display.send_data(U8(&data[0..len]))
        } else {
            Ok(())
        }
    }
}
