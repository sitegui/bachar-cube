use crate::position::{BITS_PER_PIECE, LAST_PIECE_MASK};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct ScorableLayer {
    pieces: u64,
    num_pieces: u32,
}

impl ScorableLayer {
    /// Split a full position into its two layers
    ///
    /// # Panics
    /// It will panic if the layers cannot be correctly constructed
    pub fn split(pieces: u64) -> (Self, Self) {
        // Check if the pattern can finish with a layer by counting how many big pieces there
        // are in the right-most positions. A layer can be made of either:
        // - 6 big pieces
        // - 5 big and 2 small
        // - 4 big and 4 small
        // - 3 big and 6 small
        // - 2 big and 8 small
        // Each piece is 4 bits, and the mask 0b0010 reveals whether it's a big piece.
        const MASK_LAST_6_SIZES: u64 = 0x22_2222;
        const MASK_LAST_7_SIZES: u64 = 0x222_2222;
        const MASK_LAST_8_SIZES: u64 = 0x2222_2222;
        const MASK_LAST_9_SIZES: u64 = 0x2_2222_2222;
        const MASK_LAST_10_SIZES: u64 = 0x22_2222_2222;

        let bottom_num_pieces = if pieces & MASK_LAST_6_SIZES == MASK_LAST_6_SIZES {
            6
        } else if (pieces & MASK_LAST_7_SIZES).count_ones() == 5 {
            7
        } else if (pieces & MASK_LAST_8_SIZES).count_ones() == 4 {
            8
        } else if (pieces & MASK_LAST_9_SIZES).count_ones() == 3 {
            9
        } else if (pieces & MASK_LAST_10_SIZES).count_ones() == 2 {
            10
        } else {
            unreachable!()
        };

        let top_pieces = pieces >> (BITS_PER_PIECE * bottom_num_pieces);
        let bottom_pieces = pieces ^ (top_pieces << (BITS_PER_PIECE * bottom_num_pieces));
        (
            ScorableLayer {
                pieces: top_pieces,
                num_pieces: 16 - bottom_num_pieces,
            },
            ScorableLayer {
                pieces: bottom_pieces,
                num_pieces: bottom_num_pieces,
            },
        )
    }

    pub fn score(self) -> u8 {
        let mut score = 0;
        let mut bits = self.pieces;
        let left_most_piece_shift = BITS_PER_PIECE * self.num_pieces - BITS_PER_PIECE;

        for _ in 0..self.num_pieces {
            let last_piece = bits & LAST_PIECE_MASK;
            let second_last_piece = (bits >> BITS_PER_PIECE) & LAST_PIECE_MASK;

            // Each piece has 4 bits, the 3 MSB represent a circular sequence number of a given
            // external color (white/yellow).
            let successor_second_last_piece = (second_last_piece + 0b0010) & LAST_PIECE_MASK;

            score += (last_piece == successor_second_last_piece) as u8;

            bits = (bits >> 4) | (last_piece << left_most_piece_shift);
        }

        score
    }
}
