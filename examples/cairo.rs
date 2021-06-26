use cairo::{Context, Format, ImageSurface};
use std::fs::File;
use std::{thread, time};
use {
    display_interface_spi::SPIInterfaceNoCS,
    linux_embedded_hal::Delay,
    rppal::{
        gpio::Gpio,
        spi::{Bus, Mode, SlaveSelect, Spi},
    },
};

use ssd1351;

const DISPLAY_WIDTH: f64 = 128.0;
const DISPLAY_HEIGHT: f64 = 128.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
 
    let sleep_duration = time::Duration::from_secs(30);

    // Configure gpio
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 19660800, Mode::Mode0).unwrap();
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(24).unwrap().into_output();
    let mut rst = gpio.get(25).unwrap().into_output();

    // Init SPI
    let spii = SPIInterfaceNoCS::new(spi, dc);
    let mut disp = ssd1351::display::Ssd1351::new(spii);

    // Reset & init
    disp.reset(&mut rst, &mut Delay).unwrap();
    disp.turn_on().unwrap();

    // Initialise cairo
    let mut surface =
        ImageSurface::create(Format::Rgb16_565, 128, 128).expect("Couldn’t create surface");

    let mut context = Context::new(&mut surface)?;

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint()?;

    println!("Draw text");
    context.select_font_face("serif", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    context.set_font_size(20.0);
    context.set_source_rgb(1.0, 1.0, 1.0);

    let text_to_display = "22/11/1890";
    let text_extent = context.text_extents(&text_to_display)?;
    let x_offset = (DISPLAY_WIDTH - text_extent.width) / 2.0;
    let y_offset = (DISPLAY_HEIGHT + text_extent.height) / 2.0;
    context.move_to(x_offset, y_offset);
    context.show_text(text_to_display)?;

    let mut file = File::create("cairo_output.png").expect("Couldn’t create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");

    drop(context);
    let mut data = surface.data()?;
    disp.draw(&data);
    drop(data);
    disp.flush();

    thread::sleep(sleep_duration);

    println!("Draw icon");
    context = Context::new(&mut surface)?;
    context.set_source_rgb(0.0, 0.0, 0.0);
    context.paint()?;

    context.select_font_face(
        "weathericons",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Normal,
    );
    context.set_font_size(90.0);
    context.set_source_rgb(0.0, 0.0, 1.0);

    let weather_icon_char = '\u{f051}'.to_string();
    let text_extent = context.text_extents(&weather_icon_char)?;
    let x_offset = (DISPLAY_WIDTH - text_extent.width) / 2.0;
    let y_offset = (DISPLAY_HEIGHT - text_extent.height) / 2.0 - text_extent.y_bearing;

    context.move_to(x_offset, y_offset);
    context.show_text(&weather_icon_char)?;

    let mut file = File::create("cairo_output.png").expect("Couldn’t create file");
    surface
        .write_to_png(&mut file)
        .expect("Couldn’t write to png");

    drop(context);
    let mut data = surface.data()?;
    disp.draw(&data);
    drop(data);
    disp.flush();

    thread::sleep(sleep_duration);

    disp.turn_off();

    Ok(())
}
