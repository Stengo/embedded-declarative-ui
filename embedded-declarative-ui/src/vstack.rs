use crate::{alignment::HorizontalAlignment, draw_target::LayoutDrawTarget, view::View};
use core::{cmp::max, marker::PhantomData};
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};

pub struct VStack<Views, Color>
where
    Views: VViewTuple<Color>,
    Color: PixelColor,
{
    spacing: u32,
    horizontal_alignment: HorizontalAlignment,
    views: Views,
    _marker: PhantomData<Color>,
}

impl<Views, Color> VStack<Views, Color>
where
    Views: VViewTuple<Color>,
    Color: PixelColor,
{
    pub fn new(spacing: u32, horizontal_alignment: HorizontalAlignment, views: Views) -> Self {
        Self {
            spacing,
            horizontal_alignment,
            views,
            _marker: PhantomData,
        }
    }
}

impl<Views, Color> View<Color> for VStack<Views, Color>
where
    Views: VViewTuple<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        self.views
            .draw_all(size, self.spacing, self.horizontal_alignment, draw_target)
            .await;
    }

    async fn size(&self, available_size: Size) -> Size {
        self.views.total_size(available_size, self.spacing).await
    }
}

pub trait VViewTuple<Color: PixelColor> {
    async fn total_size(&self, available_size: Size, spacing: u32) -> Size;

    async fn draw_all<Target, Error>(
        &self,
        available_size: Size,
        spacing: u32,
        horizontal_alignment: HorizontalAlignment,
        draw_target: &mut LayoutDrawTarget<'_, Target>,
    ) where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static;
}

macro_rules! impl_vview_tuple {
    ($($name:ident),*) => {
        impl<Color, $($name),*> VViewTuple<Color> for ($($name,)*)
        where
            Color: PixelColor,
            $($name: View<Color>),*
        {
            async fn total_size(&self, available_size: Size, spacing: u32) -> Size {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                let mut total_height = 0;
                let mut max_width = 0;
                let mut count = 0;

                $(
                    let remaining_size = Size {
                        width: available_size.width,
                        height: available_size.height - total_height,
                    };
                    let s = $name.size(remaining_size).await;
                    total_height += s.height;
                    max_width = max(max_width, s.width);
                    count += 1;
                )*

                if count > 1 {
                    total_height += spacing * (count - 1);
                }

                Size { width: max_width, height: total_height }
            }

            #[allow(unused_assignments)]
            async fn draw_all<Target, Error>(
                &self,
                available_size: Size,
                spacing: u32,
                horizontal_alignment: HorizontalAlignment,
                draw_target: &mut LayoutDrawTarget<'_, Target>,
            ) where
                Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
                Error: 'static,
            {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                let mut current_y_offset = 0;

                $(
                    let remaining_size = Size {
                        width: available_size.width,
                        height: available_size.height - current_y_offset,
                    };
                    let view_size = $name.size(remaining_size).await;

                    let x_offset = match horizontal_alignment {
                        HorizontalAlignment::Left => 0,
                        HorizontalAlignment::Right => available_size.width - view_size.width,
                        HorizontalAlignment::Center => (available_size.width - view_size.width) / 2,
                    };

                    let mut child_target = LayoutDrawTarget {
                        original_draw_target: draw_target.original_draw_target,
                        offset: Point {
                            x: draw_target.offset.x + x_offset as i32,
                            y: draw_target.offset.y + current_y_offset as i32,
                        },
                    };

                    $name.draw(view_size, &mut child_target).await;
                    current_y_offset += view_size.height + spacing;
                )*
            }
        }
    };
}

impl_vview_tuple!(V1);
impl_vview_tuple!(V1, V2);
impl_vview_tuple!(V1, V2, V3);
impl_vview_tuple!(V1, V2, V3, V4);
impl_vview_tuple!(V1, V2, V3, V4, V5);
impl_vview_tuple!(V1, V2, V3, V4, V5, V6);
impl_vview_tuple!(V1, V2, V3, V4, V5, V6, V7);
impl_vview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8);
impl_vview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9);
impl_vview_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
