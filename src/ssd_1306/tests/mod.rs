use anyhow::*;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_hal::delay::DelayNs;
use rand::random;
use crate::helpers::{*, graphics::*};
use crate::helpers::graphics::lines::DrawLine;

#[allow(dead_code)]
pub fn draw_lines<D>(
    display: &mut D,
    delay: &mut impl DelayNs,
) -> Result<(), Error>
    where 
        D: DrawTarget<Color = BinaryColor, Error = display_interface::DisplayError> + OriginDimensions + DrawLine<BinaryColor>,
{
    display.clear(BinaryColor::Off).expect("Failed to clear display");
    let height = display.height() as i32;
    let width = display.width() as i32;
    let mut line = lines::Line::<BinaryColor>::default();
    line.with_color(BinaryColor::On)
        .with_stroke(1);
    for w in (0..display.width()).filter(|x| x % 4 == 0) {
        line.with_xy1(0, 0)
            .with_xy2(w as i32, height - 1);
        display.draw_single_line(&line).into_anyhow()?;
        delay.delay_ms(20);
    }
    for h in (0..display.height()).filter(|x| x % 4 == 0) {
        line.with_xy1(0, 0)
            .with_xy2(width - 1, h as i32);
        display.draw_single_line(&line).into_anyhow()?;
        delay.delay_ms(20);
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn draw_disco_lines<D>(
    display: &mut D,
    delay: &mut impl DelayNs,
    repeats: u32,
) -> Result<(), Error>
where
    D: DrawTarget<Color = BinaryColor, Error = display_interface::DisplayError> + OriginDimensions + DrawLine<BinaryColor>,
{
    let height = display.height() as i32;
    let width = display.width() as i32;
    let clear = |display: &mut D| display.clear(BinaryColor::Off).expect("Failed to clear display");
    clear(display);
    for _ in 0..repeats {
        let mut line = lines::Line::<BinaryColor>::default();
        line.with_color(BinaryColor::On)
            .with_stroke(1);
        let start_y = random();
        let random = random::<i32>();
        let current_line = if start_y {
            line.with_xy1(random % width, 0)
                .with_xy2(random % width, height)
        } else {
            line.with_xy1(0, random % height)
                .with_xy2(width, random % height)
        };
        display.draw_single_line(current_line).into_anyhow()?;
        delay.delay_ms(24);
        clear(display);
        delay.delay_ms(6);
    }
    Ok(())
}