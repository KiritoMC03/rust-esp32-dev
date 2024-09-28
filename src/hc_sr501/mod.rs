use std::fmt::Display;
use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{InputPin, OutputPin, Pin, PinDriver};
use esp_idf_hal::gpio::{Input, InterruptType, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_sys::EspError;

use crate::helpers;

/// Datasheet: https://www.mpja.com/download/31227sc.pdf
#[allow(dead_code)]
pub struct HCSR501<'a, T: Pin> {
    pin: PinDriver<'a, T, Input>,
    moves_count: u32,
}

#[allow(dead_code)]
pub struct Movement<'b, T: Pin> {
    id: u32,
    source: &'b HCSR501<'b, T>,
    start_time: u128,
    end_time: u128,
}

impl<T: Pin> Display for Movement<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Movement: {}, started at: {}, finished at: {}", self.id, self.start_time / 1000000, self.end_time / 1000000)
    }
}

impl<'a, T: InputPin + OutputPin> HCSR501<'a, T> {
    pub fn from_pin(pin: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
        Ok(Self {
            pin: helpers::pin::input_pin_pull_intrp(pin, Pull::Up, InterruptType::AnyEdge)?,
            moves_count: 0,
        })
    }

    pub fn check(&mut self) -> bool {
        self.pin.is_high()
    }
    
    pub fn wait_next_move(&mut self) -> Movement<T> {
        while self.pin.is_low() {
            FreeRtos::delay_ms(1)
        }
        let start_time = EspSystemTime {}.now().as_micros();
        self.moves_count += 1;
        Movement::new(self.moves_count, self, start_time)
    }
}

impl<'b, T: InputPin + OutputPin> Movement<'b, T> {
    pub fn new(id: u32, source: &'b HCSR501<'b, T>, start_time: u128) -> Self {
        Self {
            id,
            source,
            start_time,
            end_time: 0,
        }
    }
    
    pub fn join(&mut self) {
        if self.id != self.source.moves_count {
            return
        }
        while self.source.pin.is_high() {
            FreeRtos::delay_ms(1)
        }
        self.end_time = EspSystemTime {}.now().as_micros();
    }
}