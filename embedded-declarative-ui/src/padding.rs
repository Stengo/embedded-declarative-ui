use crate::{draw_target::LayoutDrawTarget, view::View};
use core::marker::PhantomData;
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};

pub struct Padding<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    inner_view: InnerView,
    _marker: PhantomData<Color>,
}

#[allow(dead_code)]
impl<InnerView, Color> Padding<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    pub fn new(left: u32, top: u32, right: u32, bottom: u32, inner_view: InnerView) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            inner_view,
            _marker: PhantomData,
        }
    }

    pub fn top(top: u32, inner_view: InnerView) -> Self {
        Self::new(0, top, 0, 0, inner_view)
    }

    pub fn bottom(bottom: u32, inner_view: InnerView) -> Self {
        Self::new(0, 0, 0, bottom, inner_view)
    }

    pub fn left(left: u32, inner_view: InnerView) -> Self {
        Self::new(left, 0, 0, 0, inner_view)
    }

    pub fn right(right: u32, inner_view: InnerView) -> Self {
        Self::new(0, 0, right, 0, inner_view)
    }

    pub fn horizontal(horizontal: u32, inner_view: InnerView) -> Self {
        Self::new(horizontal, 0, horizontal, 0, inner_view)
    }

    pub fn vertical(vertical: u32, inner_view: InnerView) -> Self {
        Self::new(0, vertical, 0, vertical, inner_view)
    }

    pub fn all(all: u32, inner_view: InnerView) -> Self {
        Self::new(all, all, all, all, inner_view)
    }
}

impl<Color, InnerView> View<Color> for Padding<InnerView, Color>
where
    InnerView: View<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        let content_size = Size::new(
            size.width - (self.left + self.right),
            size.height - (self.top + self.bottom),
        );

        let mut offset_draw_target = LayoutDrawTarget {
            original_draw_target: draw_target.original_draw_target,
            offset: Point {
                x: draw_target.offset.x + self.left as i32,
                y: draw_target.offset.y + self.top as i32,
            },
        };

        self.inner_view
            .draw(content_size, &mut offset_draw_target)
            .await;
    }

    async fn size(&self, available_size: Size) -> Size {
        let content_available_size = Size::new(
            available_size.width - (self.left + self.right),
            available_size.height - (self.top + self.bottom),
        );

        let content_size = self.inner_view.size(content_available_size).await;

        Size::new(
            content_size.width + self.left + self.right,
            content_size.height + self.top + self.bottom,
        )
    }
}
