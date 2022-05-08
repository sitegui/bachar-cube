use crate::{OuterLayer, OuterPiece};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct Position {
    pub top: OuterLayer,
    pub middle_solved: bool,
    pub bottom: OuterLayer,
}

impl Position {
    pub fn solved() -> Position {
        Position {
            top: OuterLayer::new([
                OuterPiece::WhiteRedBlue1,
                OuterPiece::WhiteRedBlue2,
                OuterPiece::WhiteBlue,
                OuterPiece::WhiteBlueOrange1,
                OuterPiece::WhiteBlueOrange2,
                OuterPiece::WhiteOrange,
                OuterPiece::WhiteOrangeGreen1,
                OuterPiece::WhiteOrangeGreen2,
                OuterPiece::WhiteGreen,
                OuterPiece::WhiteGreenRed1,
                OuterPiece::WhiteGreenRed2,
                OuterPiece::WhiteRed,
            ]),
            middle_solved: true,
            bottom: OuterLayer::new([
                OuterPiece::YellowOrange,
                OuterPiece::YellowOrangeBlue1,
                OuterPiece::YellowOrangeBlue2,
                OuterPiece::YellowBlue,
                OuterPiece::YellowBlueRed1,
                OuterPiece::YellowBlueRed2,
                OuterPiece::YellowRed,
                OuterPiece::YellowRedGreen1,
                OuterPiece::YellowRedGreen2,
                OuterPiece::YellowGreen,
                OuterPiece::YellowGreenOrange1,
                OuterPiece::YellowGreenOrange2,
            ]),
        }
    }

    pub fn for_each_movement(&self, mut f: impl FnMut(Self)) {
        self.top.for_each_movement(|new_top| {
            f(Position {
                top: new_top,
                middle_solved: self.middle_solved,
                bottom: self.bottom,
            })
        });

        let (flipped_top, flipped_bottom) = self.top.flip(self.bottom);
        f(Position {
            top: flipped_top,
            middle_solved: !self.middle_solved,
            bottom: flipped_bottom,
        });

        self.bottom.for_each_movement(|new_bottom| {
            f(Position {
                top: self.top,
                middle_solved: self.middle_solved,
                bottom: new_bottom,
            })
        });
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.top, self.middle_solved, self.bottom)
    }
}
