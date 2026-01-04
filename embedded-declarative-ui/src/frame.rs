use crate::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    draw_target::LayoutDrawTarget,
    view::View,
};
use core::marker::PhantomData;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};

pub enum Dimension {
    Min,
    Max,
    Constant(u32),
    Fraction(f32),
}

pub struct Frame<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    width: Dimension,
    height: Dimension,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    inner_view: InnerView,
    _marker: PhantomData<Color>,
}

impl<InnerView, Color> Frame<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    pub fn new(
        width: Dimension,
        height: Dimension,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        inner_view: InnerView,
    ) -> Frame<InnerView, Color> {
        Frame {
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            inner_view,
            _marker: PhantomData,
        }
    }

    async fn resolve_size(&self, available_size: Size) -> Size {
        let w = match self.width {
            Dimension::Max => available_size.width,
            Dimension::Constant(c) => c,
            Dimension::Min => self.inner_view.size(available_size).await.width,
            Dimension::Fraction(f) => (available_size.width as f32 * f) as u32,
        };

        let h = match self.height {
            Dimension::Max => available_size.height,
            Dimension::Constant(c) => c,
            Dimension::Min => {
                self.inner_view
                    .size(Size::new(w, available_size.height))
                    .await
                    .height
            }
            Dimension::Fraction(f) => (available_size.height as f32 * f) as u32,
        };

        Size::new(w, h)
    }
}

impl<Color, InnerView> View<Color> for Frame<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        let content_size = self.inner_view.size(size).await;

        let x_offset = match self.horizontal_alignment {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Right => size.width as i32 - content_size.width as i32,
            HorizontalAlignment::Center => (size.width as i32 - content_size.width as i32) / 2,
        };

        let y_offset = match self.vertical_alignment {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Bottom => size.height as i32 - content_size.height as i32,
            VerticalAlignment::Center => (size.height as i32 - content_size.height as i32) / 2,
        };

        let mut offset_draw_target = LayoutDrawTarget {
            original_draw_target: draw_target.original_draw_target,
            offset: Point {
                x: draw_target.offset.x + x_offset,
                y: draw_target.offset.y + y_offset,
            },
        };

        self.inner_view
            .draw(content_size, &mut offset_draw_target)
            .await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.resolve_size(available_size).await
    }
}
