use crate::{OuterLayer, OuterPiece};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct Position {
    pub top: OuterLayer,
    pub middle_solved: bool,
    pub bottom: OuterLayer,
}

#[derive(Debug, Clone, Copy)]
pub struct MovementKind {
    top: bool,
    middle: bool,
    bottom: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    pub position: Position,
    pub next_kind: MovementKind,
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

    pub fn for_each_movement(&self, kind: MovementKind, mut f: impl FnMut(Movement)) {
        if kind.top {
            self.top.for_each_movement(|new_top| {
                let new_position = Position {
                    top: new_top,
                    middle_solved: self.middle_solved,
                    bottom: self.bottom,
                };
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind {
                        top: false,
                        middle: true,
                        bottom: kind.bottom,
                    },
                })
            });
        }

        if kind.middle {
            let (flipped_top, flipped_bottom) = self.top.flip(self.bottom);
            let new_position = Position {
                top: flipped_top,
                middle_solved: !self.middle_solved,
                bottom: flipped_bottom,
            };
            f(Movement {
                position: new_position,
                next_kind: MovementKind {
                    top: true,
                    middle: false,
                    bottom: true,
                },
            });
        }

        if kind.bottom {
            self.bottom.for_each_movement(|new_bottom| {
                let new_position = Position {
                    top: self.top,
                    middle_solved: self.middle_solved,
                    bottom: new_bottom,
                };
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind {
                        top: kind.top,
                        middle: true,
                        bottom: false,
                    },
                })
            });
        }
    }
}

impl MovementKind {
    pub const ALL: MovementKind = MovementKind {
        top: true,
        middle: true,
        bottom: true,
    };
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.top, self.middle_solved, self.bottom)
    }
}
