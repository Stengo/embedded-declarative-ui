use crate::{draw_target::LayoutDrawTarget, view::View};
use core::{fmt::Display, marker::PhantomData};
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Point, Size};
use u8g2_fonts::{
    Font, FontRenderer,
    types::{FontColor, VerticalPosition},
};

pub struct Text<Color, FONT, CONTENT>
where
    Color: PixelColor,
    FONT: Font,
    CONTENT: Display,
{
    content: CONTENT,
    color: Color,
    _marker: PhantomData<FONT>,
}

impl<Color, FONT, CONTENT> Text<Color, FONT, CONTENT>
where
    Color: PixelColor,
    FONT: Font,
    CONTENT: Display,
{
    pub fn new(content: CONTENT, color: Color, _font: FONT) -> Self {
        Text {
            content,
            color,
            _marker: PhantomData,
        }
    }
}

impl<Color, FONT, CONTENT> View<Color> for Text<Color, FONT, CONTENT>
where
    Color: PixelColor,
    FONT: Font,
    CONTENT: Display,
{
    async fn draw<Target, Error>(&self, _size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        let renderer = FontRenderer::new::<FONT>();

        _ = renderer.render(
            format_args!("{}", self.content),
            Point::new(0, 0),
            VerticalPosition::Top,
            FontColor::Transparent(self.color),
            draw_target,
        );
    }

    async fn size(&self, _available_size: Size) -> Size {
        let renderer = FontRenderer::new::<FONT>();
        let dimensions = renderer
            .get_rendered_dimensions(
                format_args!("{}", self.content),
                Point::new(0, 0),
                VerticalPosition::Top,
            )
            .unwrap();

        let bounding_box = dimensions.bounding_box.unwrap_or_default();
        Size {
            width: dimensions.advance.x as u32,
            height: (bounding_box.size.height as i32 + bounding_box.top_left.y) as u32,
        }
    }
}
