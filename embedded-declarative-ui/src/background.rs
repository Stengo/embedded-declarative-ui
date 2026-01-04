use crate::{draw_target::LayoutDrawTarget, view::View};
use core::marker::PhantomData;
use embedded_graphics::{
    Drawable,
    prelude::{DrawTarget, OriginDimensions, PixelColor, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
};

pub struct Background<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    color: Color,
    inner_view: InnerView,
    _marker: PhantomData<Color>,
}

impl<InnerView, Color> Background<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    pub fn new(color: Color, inner_view: InnerView) -> Self {
        Self {
            color,
            inner_view,
            _marker: PhantomData,
        }
    }
}

impl<Color, InnerView> View<Color> for Background<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        let rectangle = Rectangle::new(draw_target.offset, size);
        let style = PrimitiveStyle::with_fill(self.color);

        _ = rectangle
            .into_styled(style)
            .draw(draw_target.original_draw_target);

        self.inner_view.draw(size, draw_target).await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.inner_view.size(available_size).await
    }
}
