pub mod lines;

use anyhow::*;
use display_interface::DisplayError;
use embedded_graphics::geometry::OriginDimensions;

#[allow(dead_code)]
pub trait OriginDimensionsDetailed : OriginDimensions {
    fn width(&self) -> u32 {
        self.size().width
    }

    fn height(&self) -> u32 {
        self.size().height
    }
}
impl<T> OriginDimensionsDetailed for T where T: OriginDimensions {}

#[allow(dead_code)]
pub trait Flush {
    fn flush(&mut self) -> Result<(), DisplayError>;
}