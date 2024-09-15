mod lcd;
mod blink;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use hd44780_driver::Display;

fn main() -> anyhow::Result<()> {
    prepare();
    let peripherals = Peripherals::take().unwrap();
    let (mut lcd, mut delay) = lcd::get_cur_lcd(peripherals.pins.gpio21, peripherals.pins.gpio22, peripherals.i2c0)?;
    lcd.reset(&mut delay).expect("Can't reset display");
    lcd.clear(&mut delay).expect("Can't clear display");
    lcd::lcd_on(&mut lcd, Display::On, &mut delay);
    
    let mut line_num = 0;
    let mut type_new = |str| {
        if line_num == 2 {
            lcd.clear(&mut delay).expect("Can't clear display");
            lcd.set_cursor_pos(0, &mut delay).expect("Can't reset cursor position");
            line_num = 0;
        }
        lcd::smooth_type(&mut lcd, &mut delay, str);
        lcd.set_cursor_pos(40, &mut delay).expect("Can't return home");
        FreeRtos::delay_ms(1000);
        line_num+=1;
    };

    type_new("Hi!");
    type_new("I love You!");
    type_new("And you?");
    
    let mut led = blink::Led::from_pin(peripherals.pins.gpio2).expect("Can't create LED");
    led.blink_inf(&blink::default_seq()).expect("Can't blink LED");
    
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