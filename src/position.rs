use crate::outer_layer::OuterLayer;
use crate::outer_piece::OuterPiece;
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct Position {
    top: OuterLayer,
    middle_solved: bool,
    bottom: OuterLayer,
}

/// Represents the allowed movements from this position
///
/// Bit 0 = can rotate top
/// Bit 1 = can flip
/// Bit 2 = can rotate bottom
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MovementKind(u8);

impl MovementKind {}

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
    pub fn with_layers(top: OuterLayer, middle_solved: bool, bottom: OuterLayer) -> Self {
        Position {
            top,
            middle_solved,
            bottom,
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
        if kind.rotate_top() {
            self.top.for_each_movement(|new_top, shift| {
                let new_position = Position::with_layers(new_top, self.middle_solved, self.bottom);
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind::new(false, true, kind.rotate_bottom()),
                    change: MovementChange::RotateTop(shift),
                })
            });
        }

        if kind.flip() {
            f(Movement {
                position: self.flipped(),
                next_kind: MovementKind::new(true, false, true),
                change: MovementChange::Flip,
            });
        }

        if kind.rotate_bottom() {
            self.bottom.for_each_movement(|new_bottom, shift| {
                let new_position = Position::with_layers(self.top, self.middle_solved, new_bottom);
                f(Movement {
                    position: new_position,
                    next_kind: MovementKind::new(kind.rotate_top(), true, false),
                    change: MovementChange::RotateBottom(shift),
                })
            });
        }
    }

    /// Return how many pieces are relatively well placed
    pub fn solved_score(self) -> u8 {
        self.top.solved_score() + self.middle_solved as u8 + self.bottom.solved_score()
    }

    /// Return a very dense representation of the position, meant to represent very efficiently
    /// this position.
    ///
    /// Each top piece is encoded in order, then each bottom piece is encoded in order, except the
    /// last one (because it's a redundant information). Each piece is encoded as 4 bits.
    ///
    /// The last 4 bits are used to represent the middle piece.
    pub fn as_bytes(self) -> u64 {
        let mut encoded = 0;

        // There are exactly 16 pieces with id in total, so this will fill in all the 64 bits
        for piece in self.top.pieces() {
            if let Some(id) = piece.id() {
                encoded <<= 4;
                encoded |= id as u64;
            }
        }
        for piece in self.bottom.pieces() {
            if let Some(id) = piece.id() {
                encoded <<= 4;
                encoded |= id as u64;
            }
        }

        // Ditch the last 4 bits
        encoded >>= 4;
        encoded <<= 4;

        // The last 4 bits represent the middle piece
        encoded |= self.middle_solved as u64;

        encoded
    }
}

impl MovementKind {
    pub const ALL: MovementKind = MovementKind::new(true, true, true);

    const fn new(rotate_top: bool, flip: bool, rotate_bottom: bool) -> Self {
        MovementKind((rotate_top as u8) << 2 | (flip as u8) << 1 | (rotate_bottom as u8))
    }

    fn rotate_top(self) -> bool {
        self.0 & 0b100 != 0
    }

    fn flip(self) -> bool {
        self.0 & 0b10 != 0
    }

    fn rotate_bottom(self) -> bool {
        self.0 & 0b1 != 0
    }
}

impl MovementChange {
    pub fn inversed(self) -> Self {
        use MovementChange::*;
        match self {
            Flip => Flip,
            RotateTop(n) => RotateTop(12 - n),
            RotateBottom(n) => RotateBottom(12 - n),
        }
    }
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
