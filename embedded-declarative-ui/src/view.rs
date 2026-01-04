use crate::draw_target::LayoutDrawTarget;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, Size};

pub trait View<Color> {
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static;

    async fn size(&self, available_size: Size) -> Size;
}

impl<'a, C, V> View<C> for &'a V
where
    V: View<C>,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = C, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        (**self).draw(size, draw_target).await
    }

    async fn size(&self, available_size: Size) -> Size {
        (**self).size(available_size).await
    }
}

pub trait CompositeView<Color>
where
    Color: embedded_graphics::pixelcolor::PixelColor,
{
    async fn body(&self) -> impl View<Color>;
}
