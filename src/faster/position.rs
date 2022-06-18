use super::piece::Piece;
use crate::layer::Layer;
use itertools::Itertools;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pieces: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Change {
    top_before: u8,
    bottom_before: u8,
    top_after: u8,
    bottom_after: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    change: Change,
    position: Position,
}

/// A helper struct used by [`Position::neighbours()`] to avoid intermediate allocations in every
/// call.
#[derive(Debug, Clone)]
pub struct NeighboursStack {
    neighbours: Vec<Movement>,
    top_before_rotations: Vec<(Layer, u8)>,
    bottom_before_rotations: Vec<(Layer, u8)>,
    top_after_rotations: Vec<(Layer, u8)>,
    bottom_after_rotations: Vec<(Layer, u8)>,
}

impl Position {
    pub fn solved() -> Self {
        use Piece::*;

        Self::from_pieces([
            WhiteRedBlue,
            WhiteBlue,
            WhiteBlueOrange,
            WhiteOrange,
            WhiteOrangeGreen,
            WhiteGreen,
            WhiteGreenRed,
            WhiteRed,
            YellowOrange,
            YellowOrangeBlue,
            YellowBlue,
            YellowBlueRed,
            YellowRed,
            YellowRedGreen,
            YellowGreen,
            YellowGreenOrange,
        ])
    }

    pub fn from_pieces(pieces: [Piece; 16]) -> Self {
        assert_eq!(pieces.iter().unique().count(), 16);

        let mut as_bits = 0;
        for piece in pieces {
            as_bits <<= 4;
            as_bits |= piece.as_bits();
        }

        Position { pieces: as_bits }
    }

    pub fn pieces(&self) -> [Piece; 16] {
        let mut pieces = [Piece::YellowRedGreen; 16];

        for (i, piece) in pieces.iter_mut().enumerate() {
            let shift = 60 - 4 * i;
            *piece = Piece::from_bits((self.pieces >> shift) & 0xF);
        }

        pieces
    }

    pub fn neighbours(&self, stack: &mut NeighboursStack) {
        stack.neighbours.clear();

        let (top, bottom) = Layer::split(self.pieces);
        top.rotations(&mut stack.top_before_rotations);
        bottom.rotations(&mut stack.bottom_before_rotations);

        for &(rotated_top, top_before) in &stack.top_before_rotations {
            for &(rotated_bottom, bottom_before) in &stack.bottom_before_rotations {
                let (flipped_top, flipped_bottom) = Layer::flip(rotated_top, rotated_bottom);

                flipped_top.rotations(&mut stack.top_after_rotations);
                flipped_bottom.rotations(&mut stack.bottom_after_rotations);

                for &(rotated_top, top_after) in &stack.top_after_rotations {
                    for &(rotated_bottom, bottom_after) in &stack.bottom_after_rotations {
                        let (flipped_top, flipped_bottom) =
                            Layer::flip(rotated_top, rotated_bottom);

                        stack.neighbours.push(Movement {
                            change: Change {
                                top_before,
                                bottom_before,
                                top_after,
                                bottom_after,
                            },
                            position: Position {
                                pieces: Layer::join(flipped_top, flipped_bottom),
                            },
                        });
                    }
                }
            }
        }
    }
}

impl NeighboursStack {
    pub fn new() -> Self {
        NeighboursStack {
            // Worst-case scenario: each one of `top_before`, `bottom_before`, `top_after`,
            // `bottom_after` goes from 0 to 9 (inclusive).
            neighbours: Vec::with_capacity(10_000),
            top_before_rotations: Vec::with_capacity(10),
            bottom_before_rotations: Vec::with_capacity(10),
            top_after_rotations: Vec::with_capacity(10),
            bottom_after_rotations: Vec::with_capacity(10),
        }
    }

    pub fn neighbours(&self) -> &[Movement] {
        &self.neighbours
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pieces = self.pieces();
        let mut size = 0;

        for piece in pieces {
            if size == 6 || size == 12 || size == 18 {
                f.write_str(" | ")?;
            } else if size != 0 {
                f.write_char(' ')?;
            }

            write!(f, "{}", piece)?;

            size += piece.size();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, HashSet};

    #[test]
    fn pieces() {
        use Piece::*;

        let solved = Position::solved();
        let pieces = solved.pieces();

        assert_eq!(
            pieces,
            [
                WhiteRedBlue,
                WhiteBlue,
                WhiteBlueOrange,
                WhiteOrange,
                WhiteOrangeGreen,
                WhiteGreen,
                WhiteGreenRed,
                WhiteRed,
                YellowOrange,
                YellowOrangeBlue,
                YellowBlue,
                YellowBlueRed,
                YellowRed,
                YellowRedGreen,
                YellowGreen,
                YellowGreenOrange
            ]
        );

        assert_eq!(
            solved.to_string(),
            "WRB WB WBO WO | WOG WG WGR WR | YO YOB YB YBR | YR YRG YG YGO"
        );
    }

    #[test]
    fn neighbours() {
        let solved = Position::from_pieces([
            Piece::YellowOrange,
            Piece::WhiteGreen,
            Piece::WhiteBlueOrange,
            Piece::WhiteGreenRed,
            Piece::YellowBlueRed,
            Piece::WhiteOrange,
            Piece::YellowGreenOrange,
            Piece::YellowGreen,
            Piece::WhiteRed,
            Piece::WhiteRedBlue,
            Piece::WhiteBlue,
            Piece::WhiteOrangeGreen,
            Piece::YellowRed,
            Piece::YellowRedGreen,
            Piece::YellowOrangeBlue,
            Piece::YellowBlue,
        ]);
        let mut neighbours = NeighboursStack::new();

        solved.neighbours(&mut neighbours);

        println!("{}", neighbours.neighbours().len());
        for n in neighbours.neighbours() {
            println!("{}", n.position);
        }
    }
}
