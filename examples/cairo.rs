use cairo::{Context, Format, ImageSurface};
use std::fs::File;
use std::{thread, time};
use 
    rppal::{
        gpio::Gpio,
        spi::{Bus, Mode, SlaveSelect, Spi},
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
    let  rst = gpio.get(25).unwrap().into_output();

    // Init the display
    let mut disp = ssd1351::simple_display::display::Ssd1351::new(spi,dc,rst);

    // Reset & init
    disp.reset().unwrap();
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
    let  data = surface.data()?;
    disp.update_buffer(&data);
    drop(data);
    disp.flush()?;

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
    let (r, g, b) = convert_hex_colour_to_rgb("#FF530D".to_string())?;
    context.set_source_rgb(r / 255.0, g / 255.0, b / 255.0);

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
    let  data = surface.data()?;
    disp.update_buffer(&data);
    drop(data);
    disp.flush()?;

    thread::sleep(sleep_duration);

    disp.turn_off()?;

    Ok(())
}

/// Convert from the HEX color model to the RGB color model.
pub fn convert_hex_colour_to_rgb(
    hexcolour: String,
) -> Result<(f64, f64, f64), std::num::ParseIntError> {
    let increment;
    let r: f64;
    let g: f64;
    let b: f64;

    if hexcolour.chars().count() == 7 {
        increment = 1;
    } else {
        increment = 0;
    }

    let r_string: String = hexcolour.chars().skip(increment).take(2).collect();
    let g_string: String = hexcolour.chars().skip(increment + 2).take(2).collect();
    let b_string: String = hexcolour.chars().skip(increment + 4).take(2).collect();

    r = i64::from_str_radix(&r_string, 16)? as f64;
    g = i64::from_str_radix(&g_string, 16)? as f64;
    b = i64::from_str_radix(&b_string, 16)? as f64;

    Ok((r, g, b))
}
