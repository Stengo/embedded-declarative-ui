use embedded_graphics::Pixel;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point, Size};
use embedded_graphics::pixelcolor::PixelColor;

pub enum DrawError {
    Unknown,
}

pub struct LayoutDrawTarget<'a, T> {
    pub original_draw_target: &'a mut T,
    pub offset: Point,
}

impl<'a, T, Error, Color> DrawTarget for LayoutDrawTarget<'a, T>
where
    T: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
    Color: PixelColor,
{
    type Color = Color;
    type Error = DrawError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.original_draw_target
            .draw_iter(pixels.into_iter().map(|mut pixel| {
                pixel.0.x += self.offset.x;
                pixel.0.y += self.offset.y;
                pixel
            }))
            .map_err(|_| DrawError::Unknown)
    }
}

impl<'a, T> OriginDimensions for LayoutDrawTarget<'a, T>
where
    T: OriginDimensions,
{
    fn size(&self) -> Size {
        self.original_draw_target.size()
    }
}
