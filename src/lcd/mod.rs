use anyhow::Error;
use esp_idf_hal::delay::{Delay, FreeRtos};
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::i2c::I2c;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use hd44780_driver::{Display, HD44780};
use hd44780_driver::bus::I2CBus;

#[allow(dead_code)]
pub fn get_cur_lcd<'a, I2C: I2c>(
    sda: impl InputPin + OutputPin + 'a,
    scl: impl InputPin + OutputPin + 'a,
    i2c: impl Peripheral<P = I2C> + 'a) -> Result<(HD44780<I2CBus<I2cDriver<'a>>>, Delay), Error> {
    let mut i2c = I2cDriver::new(
        i2c,
        sda,
        scl,
        &I2cConfig::default().baudrate(100.kHz().into())
    )?;

    let address = (0..127).into_iter().find(|addr| {
        i2c.write(*addr, &[0x07], 25_000_000).is_ok()
    }).ok_or(Error::msg("No i2c device found"))?;

    let mut delay = Delay::new_default();
    match HD44780::new_i2c(i2c, address, &mut delay) {
        Ok(lcd) => Ok((lcd, delay)),
        Err(e) => Err(Error::msg(format!("Can't create LCD: {:?}", e)))
    }
}

#[allow(dead_code)]
pub fn lcd_on(lcd: &mut HD44780<I2CBus<I2cDriver>>, on: Display, delay: &mut Delay) {
    lcd.set_display_mode(
        hd44780_driver::DisplayMode {
            display: on,
            cursor_visibility: hd44780_driver::Cursor::Invisible,
            cursor_blink: hd44780_driver::CursorBlink::Off,
        },
        delay,
    ).expect("Can't set display mode");
}

#[allow(dead_code)]
pub fn smooth_type(lcd: &mut HD44780<I2CBus<I2cDriver>>, delay: &mut Delay, txt: &str) {
    for c in txt.chars() {
        lcd.write_str(c.to_string().as_str(), delay).expect("Can't write to display");
        FreeRtos::delay_ms(170);
    }
}