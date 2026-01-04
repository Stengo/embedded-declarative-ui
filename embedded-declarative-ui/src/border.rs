use crate::{draw_target::LayoutDrawTarget, view::View};
use core::marker::PhantomData;
use embedded_graphics::{
    Drawable,
    prelude::{DrawTarget, OriginDimensions, PixelColor, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle, RoundedRectangle, StrokeAlignment},
};

pub struct Border<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    color: Color,
    thickness: u32,
    radius: u32,
    inner_view: InnerView,
    _marker: PhantomData<Color>,
}

impl<InnerView, Color> Border<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    pub fn new(color: Color, thickness: u32, radius: u32, inner_view: InnerView) -> Self {
        Self {
            color,
            thickness,
            radius,
            inner_view,
            _marker: PhantomData,
        }
    }
}

impl<Color, InnerView> View<Color> for Border<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        let style = PrimitiveStyle::with_stroke(self.color, self.thickness);
        let mut style = style;
        style.stroke_alignment = StrokeAlignment::Inside;

        let rectangle = Rectangle::new(draw_target.offset, size);

        if self.radius > 0 {
            _ = RoundedRectangle::with_equal_corners(
                rectangle,
                Size::new(self.radius, self.radius),
            )
            .into_styled(style)
            .draw(draw_target.original_draw_target);
        } else {
            _ = rectangle
                .into_styled(style)
                .draw(draw_target.original_draw_target);
        }

        self.inner_view.draw(size, draw_target).await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.inner_view.size(available_size).await
    }
}
