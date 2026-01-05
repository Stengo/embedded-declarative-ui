use crate::{draw_target::LayoutDrawTarget, view::View};
use embedded_graphics::prelude::{DrawTarget, OriginDimensions, PixelColor, Size};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B, Color> View<Color> for Either<A, B>
where
    A: View<Color>,
    B: View<Color>,
    Color: PixelColor,
{
    async fn draw<Target, Error>(&self, size: Size, draw_target: &mut LayoutDrawTarget<'_, Target>)
    where
        Target: DrawTarget<Color = Color, Error = Error> + OriginDimensions,
        Error: 'static,
    {
        match self {
            Either::Left(view) => view.draw(size, draw_target).await,
            Either::Right(view) => view.draw(size, draw_target).await,
        }
    }

    async fn size(&self, available_size: Size) -> Size {
        match self {
            Either::Left(view) => view.size(available_size).await,
            Either::Right(view) => view.size(available_size).await,
        }
    }
}

#[macro_export]
macro_rules! __view_match_recurse {
    ($val:expr, $p1:pat => $e1:expr $(,)?) => {
        $e1
    };

    ($val:expr, $p1:pat => $e1:expr, $($tail_pat:pat => $tail_expr:expr),+ $(,)?) => {
        match $val {
            $p1 => embedded_declarative_ui::conditional::Either::Left($e1),
            _ => embedded_declarative_ui::conditional::Either::Right($crate::__view_match_recurse!($val, $($tail_pat => $tail_expr),+)),
        }
    };
}

#[macro_export]
macro_rules! view_match {
    ($val:expr, $p1:pat => $e1:expr $(,)?) => {
        $e1
    };

    ($val:expr, $p1:pat => $e1:expr, $($tail_pat:pat => $tail_expr:expr),+ $(,)?) => {
        match $val {
            $p1 => embedded_declarative_ui::conditional::Either::Left($e1),
            $( $tail_pat )|* => embedded_declarative_ui::conditional::Either::Right(
                embedded_declarative_ui::__view_match_recurse!($val, $($tail_pat => $tail_expr),+)
            ),
        }
    };
}
