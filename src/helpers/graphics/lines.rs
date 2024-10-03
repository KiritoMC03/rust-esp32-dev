use display_interface::DisplayError;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::PixelColor;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line<C: PixelColor + Default> {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub stroke: u32,
    pub color: C,
}

impl<C: PixelColor + Default> Line<C> {
    pub fn point1(&self) -> Point {
        Point::new(self.x1, self.y1)
    }

    pub fn point2(&self) -> Point {
        Point::new(self.x2, self.y2)
    }
    
    pub fn with_color(&mut self, color: C) -> &mut Self {
        self.color = color;
        self
    }
    
    pub fn with_stroke(&mut self, stroke: u32) -> &mut Self {
        self.stroke = stroke;
        self
    }

    pub fn with_point1(&mut self, point: Point) -> &mut Self {
        self.x1 = point.x;
        self.y1 = point.y;
        self
    }

    pub fn with_point2(&mut self, point: Point) -> &mut Self {
        self.x2 = point.x;
        self.y2 = point.y;
        self
    }

    pub fn with_xy1(&mut self, x: i32, y: i32) -> &mut Self {
        self.x1 = x;
        self.y1 = y;
        self
    }

    pub fn with_xy2(&mut self, x: i32, y: i32) -> &mut Self {
        self.x2 = x;
        self.y2 = y;
        self
    }
}

impl<C: PixelColor + Default> From<(Point, Point)> for Line<C> {
    fn from(value: (Point, Point)) -> Self {
        Line::<C> {
            x1: value.0.x,
            y1: value.0.y,
            x2: value.1.x,
            y2: value.1.y,
            stroke: 0,
            color: Default::default(),
        }
    }
}

impl<C: PixelColor + Default> From<(i32, i32, i32, i32)> for Line<C> {
    fn from(value: (i32, i32, i32, i32)) -> Self {
        Line::<C> {
            x1: value.0,
            y1: value.1,
            x2: value.2,
            y2: value.3,
            stroke: 0,
            color: Default::default(),
        }
    }
}

pub trait DrawLine<C: PixelColor + Default> {
    fn draw_single_line(&mut self, line: &Line<C>) -> anyhow::Result<(), DisplayError>;
}