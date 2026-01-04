use crate::{alignment::VerticalAlignment, draw_target::LayoutDrawTarget, view::View};
use core::{cmp::max, marker::PhantomData};
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};

pub struct HStack<Views, Color>
where
    Views: HViewTuple<Color>,
    Color: PixelColor,
{
    spacing: u32,
    vertical_alignment: VerticalAlignment,
    views: Views,
    _marker: PhantomData<Color>,
}

impl<Views, Color> HStack<Views, Color>
where
    Views: HViewTuple<Color>,
    Color: PixelColor,
{
    pub fn new(spacing: u32, vertical_alignment: VerticalAlignment, views: Views) -> Self {
        Self {
            spacing,
            vertical_alignment,
            views,
            _marker: PhantomData,
        }
    }
}

impl<Views, Color> View<Color> for HStack<Views, Color>
where
    Views: HViewTuple<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        self.views
            .draw_all(size, self.spacing, self.vertical_alignment, draw_target)
            .await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.views.total_size(available_size, self.spacing).await
    }
}

pub trait HViewTuple<Color: PixelColor> {
    async fn total_size(&self, available_size: Size, spacing: u32) -> Size;

    async fn draw_all<Target, Error>(
        &self,
        available_size: Size,
        spacing: u32,
        vertical_alignment: VerticalAlignment,
        draw_target: &mut LayoutDrawTarget<'_, Target>,
    ) where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static;
}

macro_rules! impl_hview_tuple {
    ($($name:ident),*) => {
        impl<Color, $($name),*> HViewTuple<Color> for ($($name,)*)
        where
            Color: PixelColor,
            $($name: View<Color>),*
        {
            async fn total_size(&self, available_size: Size, spacing: u32) -> Size {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                let mut total_width = 0;
                let mut max_height = 0;
                let mut count = 0;

                $(
                    let remaining_size = Size {
                        width: available_size.width - total_width,
                        height: available_size.height,
                    };
                    let s = $name.size(remaining_size).await;
                    total_width += s.width;
                    max_height = max(max_height, s.height);
                    count += 1;
                )*

                if count > 1 {
                    total_width += spacing * (count - 1);
                }

                Size { width: total_width, height: max_height }
            }

            #[allow(unused_assignments)]
            async fn draw_all<Target, Error>(
                &self,
                available_size: Size,
                spacing: u32,
                vertical_alignment: VerticalAlignment,
                draw_target: &mut LayoutDrawTarget<'_, Target>,
            ) where
                Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
                Error: 'static,
            {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                let mut current_x_offset = 0;

                $(
                    let remaining_size = Size {
                        width: available_size.width - current_x_offset,
                        height: available_size.height,
                    };
                    let view_size = $name.size(remaining_size).await;

                    let y_offset = match vertical_alignment {
                        VerticalAlignment::Top => 0,
                        VerticalAlignment::Bottom => available_size.height - view_size.height,
                        VerticalAlignment::Center => (available_size.height - view_size.height) / 2,
                    };

                    let mut child_target = LayoutDrawTarget {
                        original_draw_target: draw_target.original_draw_target,
                        offset: Point {
                            x: draw_target.offset.x + current_x_offset as i32,
                            y: draw_target.offset.y + y_offset as i32,
                        },
                    };

                    $name.draw(view_size, &mut child_target).await;
                    current_x_offset += view_size.width + spacing;
                )*
            }
        }
    };
}

impl_hview_tuple!(V1);
impl_hview_tuple!(V1, V2);
impl_hview_tuple!(V1, V2, V3);
impl_hview_tuple!(V1, V2, V3, V4);
impl_hview_tuple!(V1, V2, V3, V4, V5);
impl_hview_tuple!(V1, V2, V3, V4, V5, V6);
impl_hview_tuple!(V1, V2, V3, V4, V5, V6, V7);
impl_hview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8);
impl_hview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
impl_hview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
