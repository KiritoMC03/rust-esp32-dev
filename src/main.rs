mod lcd;
mod blink;
mod uuid;
mod rgb_led;
mod wifi;
mod ic_line_check;
mod hc_sr04;
mod hc_sr501;
mod helpers;
mod ssd_1306;

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor};
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use ssd1306::{prelude::*, Ssd1306};
use crate::helpers::IntoAnyhow;

fn main() -> anyhow::Result<(), anyhow::Error> {
    prepare();
    let peripherals = Peripherals::take()?;
    let mut delay = Delay::new_default();
    let mut reset = PinDriver::output(peripherals.pins.gpio17)?; // reset
    let spi = ssd_1306::spi_interface_default(
        peripherals.spi2,
        peripherals.pins.gpio18,
        peripherals.pins.gpio23,
        peripherals.pins.gpio16,
        peripherals.pins.gpio5,
    )?;
    let mut display = Ssd1306::new(
        spi,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();
    display.reset(&mut reset, &mut delay).expect("Failed to reset display");
    display.init().expect("Failed to initialize display");

    // Draw an image
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./resources/rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();
    display.flush().into_anyhow()?;
    log::info!("Image drawn");
    FreeRtos::delay_ms(1000);

    ssd_1306::tests::draw_lines(&mut display, &mut delay).expect("Failed to draw lines");
    log::info!("Lines drawn");
    FreeRtos::delay_ms(1000);

    ssd_1306::tests::draw_disco_lines(&mut display, &mut delay, 2000).expect("Failed to draw lines");
    log::info!("Disco lines drawn");
    FreeRtos::delay_ms(500);
    
    display.flush().unwrap();
    log::info!("Display written to");
    
    Ok(())
}

fn prepare() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");
    esp_idf_hal::sys::link_patches();
    log::info!("Ready");
}