use std::fmt;
use crate::{MiddleLayer, OuterLayer, OuterPiece};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    top: OuterLayer,
    middle: MiddleLayer,
    bottom: OuterLayer,
}

impl Position {
    fn solved() -> Position {
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
            middle: MiddleLayer::Solved,
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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.top, self.middle, self.bottom)
    }
}
