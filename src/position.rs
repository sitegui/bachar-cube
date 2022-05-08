use crate::{MiddleLayer, OuterLayer, OuterPiece};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct Position {
    pub top: OuterLayer,
    pub middle: bool,
    pub bottom: OuterLayer,
}

impl Position {
    pub fn solved() -> Position {
        Position {
            top: OuterLayer::new(&[
                OuterPiece::WHITE_RED_BLUE,
                OuterPiece::WHITE_BLUE,
                OuterPiece::WHITE_BLUE_ORANGE,
                OuterPiece::WHITE_ORANGE,
                OuterPiece::WHITE_ORANGE_GREEN,
                OuterPiece::WHITE_GREEN,
                OuterPiece::WHITE_GREEN_RED,
                OuterPiece::WHITE_RED,
            ]),
            middle: true,
            bottom: OuterLayer::new(&[
                OuterPiece::YELLOW_ORANGE,
                OuterPiece::YELLOW_ORANGE_BLUE,
                OuterPiece::YELLOW_BLUE,
                OuterPiece::YELLOW_BLUE_RED,
                OuterPiece::YELLOW_RED,
                OuterPiece::YELLOW_RED_GREEN,
                OuterPiece::YELLOW_GREEN,
                OuterPiece::YELLOW_GREEN_ORANGE,
            ]),
        }
    }

    pub fn for_each_movement(&self, mut f: impl FnMut(Self)) {
        self.top.for_each_movement(|new_top| {
            f(Position {
                top: new_top,
                middle: self.middle,
                bottom: self.bottom,
            })
        });

        let (flipped_top, flipped_bottom) = self.top.flip(self.bottom);
        f(Position {
            top: flipped_top,
            middle: !self.middle,
            bottom: flipped_bottom,
        });

        self.bottom.for_each_movement(|new_bottom| {
            f(Position {
                top: self.top,
                middle: self.middle,
                bottom: new_bottom,
            })
        });
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.top, self.middle, self.bottom)
    }
}
