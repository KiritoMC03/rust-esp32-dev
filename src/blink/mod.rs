use esp_idf_hal::gpio::Pin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Output, OutputPin, PinDriver};
use esp_idf_svc::sys::EspError;

#[allow(dead_code)]
pub struct Led<'a, T: Pin> {
    pin: PinDriver<'a, T, Output>,
}

#[allow(dead_code)]
impl<'a, T: OutputPin> Led<'a, T> {
    pub fn from_pin(pin: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
        Ok(Self {
            pin: PinDriver::output(pin)?,
        })
    }
    
    pub fn blink_inf(&mut self, states: &[u32]) -> Result<(), EspError> {
        loop {
            self.blink(states)?;
        }
    }
    
    pub fn blink(&mut self, states: &[u32]) -> Result<(), EspError> {
        let mut active= true;
        for state_time in states {
            if active {
                self.pin.set_high()?;
            } else {
                self.pin.set_low()?;
            }
            FreeRtos::delay_ms(*state_time);
            active = !active;
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn default_seq() -> [u32; 56] {
    [
        // Entrance
        100, 400,   // On 100 мс, Off 400 мс
        100, 400,   // On 100 мс, Off 400 мс
        100, 400,   // On 100 мс, Off 400 мс
        100, 400,   // On 100 мс, Off 400 мс

        // First accent
        100, 200,   // On 100 мс, Off 200 мс
        100, 200,   // On 100 мс, Off 200 мс
        100, 200,   // On 100 мс, Off 200 мс
        100, 200,   // On 100 мс, Off 200 мс

        // Strong accent
        300, 200,   // On 300 мс, Off 200 мс
        300, 200,   // On 300 мс, Off 200 мс
        300, 200,   // On 300 мс, Off 200 мс
        300, 200,   // On 300 мс, Off 200 мс

        // Normal accent
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс

        // Culmination
        500, 500,   // On 500 мс, Off 500 мс
        300, 200,   // On 300 мс, Off 200 мс
        300, 200,   // On 300 мс, Off 200 мс
        500, 500,   // On 500 мс, Off 500 мс

        // Repeat rhythm
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс
        100, 150,   // On 100 мс, Off 150 мс

        // Ending
        100, 400,   // On 100 мс, Off 400 мс
        100, 400,   // On 100 мс, Off 400 мс
        100, 400,   // On 100 мс, Off 400 мс
        100, 400    // On 100 мс, Off 400 мс
    ]
}