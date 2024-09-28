#[allow(dead_code)]
use anyhow::Error;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{Input, InputPin, InterruptType, Output, OutputPin, Pin, PinDriver, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_sys::EspError;
use crate::helpers;

pub  struct HCSR04<'a, TTrig: Pin, TEcho: Pin> {
    trigger: Trigger<'a, TTrig>,
    echo: Echo<'a, TEcho>,
}

pub struct Trigger<'a, T: Pin> {
    pin: PinDriver<'a, T, Output>,
}

pub struct Echo<'a, T: Pin> {
    pin: PinDriver<'a, T, Input>,
}

impl <'a, TTrig: Pin, TEcho: Pin> HCSR04<'a, TTrig, TEcho> {
    pub fn new(trigger: Trigger<'a, TTrig>, echo: Echo<'a, TEcho>) -> Self {
        Self {
            trigger,
            echo,
        }
    }
    
    pub fn measure(&mut self) -> Result<f64, Error> {
        let delay = Delay::new_default();
        // Trigger impulse
        self.trigger.pin.set_low()?;
        delay.delay_us(2);
        self.trigger.pin.set_high()?;
        delay.delay_us(10);
        self.trigger.pin.set_low()?;
        
        let mut try_count = 0;
        while self.echo.pin.is_low() {
            try_count += 1;
            if try_count > 100000 { 
                return Err(Error::msg("Echo pin is not high after 100000 us"));
            } 
        }

        let start_time = EspSystemTime {}.now().as_micros();
        while self.echo.pin.is_high() {
        }
        let end_time = EspSystemTime {}.now().as_micros();
        let pulse_duration = end_time - start_time;
        let distance_cm = (pulse_duration as f32 * 0.0343) / 2.0;

        Ok(distance_cm as f64)
    }
    
    pub fn measure_avg(&mut self, cycles: u32) -> f64 {
        let mut dist_filter = 0f64;
        for _ in 0..cycles {
            if let Ok(dist) = self.measure() {
                dist_filter += (dist - dist_filter) * 0.2
            }
        }
        dist_filter
    }
}

impl<'a, T: OutputPin> Trigger<'a, T> {
    pub fn from_pin(pin: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
        Ok(Self {
            pin: PinDriver::output(pin)?,
        })
    }
}

impl<'a, T: InputPin + OutputPin> Echo<'a, T> {
    pub fn from_pin(pin: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
        Ok(Self {
            pin: helpers::pin::input_pin_pull_intrp(pin, Pull::Down, InterruptType::AnyEdge)?,
        })
    }
}