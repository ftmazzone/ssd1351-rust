use {
    display_interface_spi::SPIInterfaceNoCS,
    linux_embedded_hal::Delay,
    rppal::{
        gpio::Gpio,
        spi::{Bus, Mode, SlaveSelect, Spi},
    },
};

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::{Alignment, Text},
};
use std::{thread, time::Duration};
use  ssd1351;

fn main() {
    // Configure gpio
    //let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 19660800, Mode::Mode0).unwrap();
    let gpio = Gpio::new().unwrap();
    //let cs = gpio.get(8).unwrap().into_output();
    let dc = gpio.get(24).unwrap().into_output();
    let mut rst = gpio.get(25).unwrap().into_output();

    // Init SPI
    let spii = SPIInterfaceNoCS::new(spi, dc);
    let mut disp = ssd1351::display::Ssd1351::new(spii);

    // Reset & init
    disp.reset(&mut rst, &mut Delay).unwrap();
    disp.turn_on().unwrap();

    // Clear the display
    disp.clear(Rgb565::BLUE).unwrap();
    //disp.flush().unwrap();
    disp.flush();

    //Write "Hello" to the display
    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::RED);

    let text = "Hello";
    Text::with_alignment(
        text,
       // disp.bounding_box().center() + Point::new(0, 15),
       Point::new(disp.bounding_box().center().x, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut disp)
    .unwrap();

    disp.flush();

    thread::sleep(Duration::from_secs(30));

    disp.reset(&mut rst, &mut Delay).unwrap();
    disp.turn_off().unwrap();
}
