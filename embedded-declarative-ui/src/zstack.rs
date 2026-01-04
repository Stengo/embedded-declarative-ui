use crate::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    draw_target::LayoutDrawTarget,
    view::View,
};
use core::{cmp::max, marker::PhantomData};
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};

pub struct ZStack<Views, Color>
where
    Views: ZViewTuple<Color>,
    Color: PixelColor,
{
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
    views: Views,
    _marker: PhantomData<Color>,
}

impl<Views, Color> ZStack<Views, Color>
where
    Views: ZViewTuple<Color>,
    Color: PixelColor,
{
    pub fn new(
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        views: Views,
    ) -> Self {
        Self {
            horizontal_alignment,
            vertical_alignment,
            views,
            _marker: PhantomData,
        }
    }
}

impl<Views, Color> View<Color> for ZStack<Views, Color>
where
    Views: ZViewTuple<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        self.views
            .draw_all(
                size,
                self.horizontal_alignment,
                self.vertical_alignment,
                draw_target,
            )
            .await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.views.total_size(available_size).await
    }
}

pub trait ZViewTuple<Color: PixelColor> {
    async fn total_size(&self, available_size: Size) -> Size;

    async fn draw_all<Target, Error>(
        &self,
        available_size: Size,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
        draw_target: &mut LayoutDrawTarget<'_, Target>,
    ) where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static;
}

macro_rules! impl_zview_tuple {
    ($($name:ident),*) => {
        impl<Color, $($name),*> ZViewTuple<Color> for ($($name,)*)
        where
            Color: PixelColor,
            $($name: View<Color>),*
        {
            async fn total_size(&self, available_size: Size) -> Size {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                let mut max_width = 0;
                let mut max_height = 0;

                $(
                    let s = $name.size(available_size).await;
                    max_width = max(max_width, s.width);
                    max_height = max(max_height, s.height);
                )*

                Size { width: max_width, height: max_height }
            }

            async fn draw_all<Target, Error>(
                &self,
                available_size: Size,
                horizontal_alignment: HorizontalAlignment,
                vertical_alignment: VerticalAlignment,
                draw_target: &mut LayoutDrawTarget<'_, Target>,
            ) where
                Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
                Error: 'static,
            {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;

                $(
                    let view_size = $name.size(available_size).await;

                    let x_offset = match horizontal_alignment {
                        HorizontalAlignment::Left => 0,
                        HorizontalAlignment::Right => available_size.width as i32 - view_size.width as i32,
                        HorizontalAlignment::Center => (available_size.width as i32 - view_size.width as i32) / 2,
                    };

                    let y_offset = match vertical_alignment {
                        VerticalAlignment::Top => 0,
                        VerticalAlignment::Bottom => available_size.height as i32 - view_size.height as i32,
                        VerticalAlignment::Center => (available_size.height as i32 - view_size.height as i32) / 2,
                    };

                    let mut child_target = LayoutDrawTarget {
                        original_draw_target: draw_target.original_draw_target,
                        offset: Point {
                            x: draw_target.offset.x + x_offset,
                            y: draw_target.offset.y + y_offset,
                        },
                    };

                    $name.draw(view_size, &mut child_target).await;
                )*
            }
        }
    };
}

impl_zview_tuple!(V1);
impl_zview_tuple!(V1, V2);
impl_zview_tuple!(V1, V2, V3);
impl_zview_tuple!(V1, V2, V3, V4);
impl_zview_tuple!(V1, V2, V3, V4, V5);
impl_zview_tuple!(V1, V2, V3, V4, V5, V6);
impl_zview_tuple!(V1, V2, V3, V4, V5, V6, V7);
impl_zview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8);
impl_zview_tuple!(V1, Bird, V3, V4, V5, V6, V7, V8, V9);
impl_zview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
