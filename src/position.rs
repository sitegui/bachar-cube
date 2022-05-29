use crate::outer_layer::{OuterLayer, OUTER_LAYER_PIECES};
use crate::outer_piece::OuterPiece;
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct Position {
    top: OuterLayer,
    middle_solved: bool,
    bottom: OuterLayer,
    solved_score: u8,
}

/// Represents the allowed movements from this position
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MovementKind {
    top: bool,
    middle: bool,
    bottom: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Movement {
    pub position: Position,
    pub next_kind: MovementKind,
    pub change: MovementChange,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MovementChange {
    Flip,
    RotateTop(u8),
    RotateBottom(u8),
}

impl Position {
    pub const MAX_SOLVED_SCORE: u8 = 25;

    pub fn with_layers(top: OuterLayer, middle_solved: bool, bottom: OuterLayer) -> Self {
        let solved_score = top.solved_score() + middle_solved as u8 + bottom.solved_score();
        Position {
            top,
            middle_solved,
            bottom,
            solved_score,
        }
    }

    pub fn solved() -> Position {
        Position::with_layers(
            OuterLayer::new([
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
            true,
            OuterLayer::new([
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
        )
    }

    pub fn flipped(self) -> Self {
        let (flipped_top, flipped_bottom) = self.top.flip(self.bottom);
        Position::with_layers(flipped_top, !self.middle_solved, flipped_bottom)
    }

    pub fn for_each_movement(&self, kind: MovementKind, mut f: impl FnMut(Movement)) {
        if kind.top {
            self.top.for_each_movement(|new_top, shift| {
                let new_position = Position::with_layers(new_top, self.middle_solved, self.bottom);
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind {
                        top: false,
                        middle: true,
                        bottom: kind.bottom,
                    },
                    change: MovementChange::RotateTop(shift),
                })
            });
        }

        if kind.middle {
            f(Movement {
                position: self.flipped(),
                next_kind: MovementKind {
                    top: true,
                    middle: false,
                    bottom: true,
                },
                change: MovementChange::Flip,
            });
        }

        if kind.bottom {
            self.bottom.for_each_movement(|new_bottom, shift| {
                let new_position = Position::with_layers(self.top, self.middle_solved, new_bottom);
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind {
                        top: kind.top,
                        middle: true,
                        bottom: false,
                    },
                    change: MovementChange::RotateBottom(shift),
                })
            });
        }
    }

    /// Return how many pieces are relatively well placed
    pub fn solved_score(self) -> u8 {
        self.top.solved_score() + self.middle_solved as u8 + self.bottom.solved_score()
    }

    pub fn as_bytes(self) -> [u8; 25] {
        let mut bytes = [u8::MAX; 25];

        bytes[..OUTER_LAYER_PIECES].copy_from_slice(&self.top.as_bytes());
        bytes[OUTER_LAYER_PIECES] = self.middle_solved as u8;
        bytes[OUTER_LAYER_PIECES + 1..].copy_from_slice(&self.bottom.as_bytes());

        bytes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_score() {
        assert_eq!(Position::solved().solved_score(), 25);
        assert_eq!(Position::solved().flipped().solved_score(), 20);
    }
}
