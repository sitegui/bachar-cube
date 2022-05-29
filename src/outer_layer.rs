use crate::OuterPiece;
use itertools::Itertools;
use std::{fmt, mem};

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct OuterLayer {
    pieces: [OuterPiece; OUTER_LAYER_PIECES],
    solved_score: u8,
}

pub const OUTER_LAYER_PIECES: usize = 12;
const OUTER_LAYER_HALF_PIECES: usize = 6;

impl OuterLayer {
    pub fn new(pieces: [OuterPiece; 12]) -> Self {
        assert!(pieces[0].can_split_before());
        assert!(pieces[OUTER_LAYER_HALF_PIECES].can_split_before());

        OuterLayer {
            pieces,
            solved_score: Self::calculate_solved_score(pieces),
        }
    }

    pub fn for_each_movement(self, mut f: impl FnMut(Self, u8)) {
        let mut call_shift = |shift: usize| {
            let mut new_pieces = [OuterPiece::Unknown; OUTER_LAYER_PIECES];
            new_pieces[..OUTER_LAYER_PIECES - shift].copy_from_slice(&self.pieces[shift..]);
            new_pieces[OUTER_LAYER_PIECES - shift..].copy_from_slice(&self.pieces[..shift]);
            f(
                OuterLayer {
                    pieces: new_pieces,
                    solved_score: self.solved_score,
                },
                shift as u8,
            );
        };

        // Shift of 6 is always possible
        call_shift(OUTER_LAYER_HALF_PIECES);

        for shift in 1..OUTER_LAYER_HALF_PIECES {
            let new_cut = shift + OUTER_LAYER_HALF_PIECES;

            if self.pieces[shift].can_split_before() && self.pieces[new_cut].can_split_before() {
                call_shift(shift);
                call_shift(new_cut);
            }
        }
    }

    pub fn flip(self, other: Self) -> (Self, Self) {
        let mut new_self_pieces = self.pieces;
        let mut new_other_pieces = other.pieces;

        // Movable pieces
        new_self_pieces[..OUTER_LAYER_HALF_PIECES]
            .copy_from_slice(&other.pieces[..OUTER_LAYER_HALF_PIECES]);
        new_other_pieces[..OUTER_LAYER_HALF_PIECES]
            .copy_from_slice(&self.pieces[..OUTER_LAYER_HALF_PIECES]);

        (
            OuterLayer {
                pieces: new_self_pieces,
                solved_score: Self::calculate_solved_score(new_self_pieces),
            },
            OuterLayer {
                pieces: new_other_pieces,
                solved_score: Self::calculate_solved_score(new_other_pieces),
            },
        )
    }

    /// Return how many pieces are relatively well placed
    pub fn solved_score(&self) -> u8 {
        self.solved_score
    }

    fn calculate_solved_score(pieces: [OuterPiece; OUTER_LAYER_PIECES]) -> u8 {
        let mut score = 0;
        for i in 1..OUTER_LAYER_PIECES {
            score += pieces[i - 1].is_followed_by(pieces[i]) as u8;
        }
        score += pieces[OUTER_LAYER_PIECES - 1].is_followed_by(pieces[0]) as u8;
        score
    }

    pub fn as_bytes(self) -> [u8; OUTER_LAYER_PIECES] {
        unsafe { mem::transmute(self.pieces) }
    }
}

impl fmt::Display for OuterLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}",
            self.pieces[..OUTER_LAYER_HALF_PIECES]
                .iter()
                .filter(|p| p.can_split_before())
                .format(","),
            self.pieces[OUTER_LAYER_HALF_PIECES..]
                .iter()
                .filter(|p| p.can_split_before())
                .format(",")
        )
    }
}
