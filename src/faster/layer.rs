use crate::piece::Piece;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Layer {
    /// The "flippable" half of the layer
    first: HalfLayer,
    /// The "fixed" half of the layer
    second: HalfLayer,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct HalfLayer {
    pieces: u64,
    num_pieces: u8,
}

impl Layer {
    /// Split a full position into it's two layers
    ///
    /// # Panics
    /// It will panic if the half-layers cannot be correctly constructed
    pub fn split(pieces: u64) -> (Self, Self) {
        let (bottom_second, remaining) = Self::extract_right_most_half_layer(pieces).unwrap();
        let (bottom_first, remaining) = Self::extract_right_most_half_layer(remaining).unwrap();
        let (top_second, top_first) = Self::extract_right_most_half_layer(remaining).unwrap();

        (
            Layer {
                first: HalfLayer {
                    pieces: top_first,
                    num_pieces: 16
                        - top_second.num_pieces
                        - bottom_first.num_pieces
                        - bottom_second.num_pieces,
                },
                second: top_second,
            },
            Layer {
                first: bottom_first,
                second: bottom_second,
            },
        )
    }

    /// Perform a flip change, by exchanging the first half-layer of each part.
    pub fn flip(top: Self, bottom: Self) -> (Self, Self) {
        (
            Layer {
                first: bottom.first,
                second: top.second,
            },
            Layer {
                first: top.first,
                second: bottom.second,
            },
        )
    }

    /// Generate all valid rotations of this layer and store them in the given vec.
    pub fn rotations(self, rotations: &mut Vec<(Layer, u8)>) {
        rotations.clear();

        // Represent the layer as an `u64`, 4 bits per piece:
        // 0 ... 0 | first | second
        let mut bits = (self.first.pieces << (4 * self.second.num_pieces)) | self.second.pieces;

        // Count how many bits away from the right the left-most piece is
        let num_pieces = self.first.num_pieces + self.second.num_pieces;
        let left_most_piece_shift = 4 * num_pieces - 4;

        // Identity is always possible
        rotations.push((self, 0));

        // Generate all other possible rotations
        for n in 1..num_pieces {
            // Take the right-most piece and push it into the left
            let right_most_piece = bits & 0xF;
            bits = (bits >> 4) | (right_most_piece << left_most_piece_shift);

            // Split the bit pattern into each half-layer
            match Self::extract_right_most_half_layer(bits) {
                None => {
                    // This is not a valid position, since no valid second half-layer can be
                    // extracted
                    continue;
                }
                Some((new_second_half, new_first_half)) => {
                    let new_layer = Layer {
                        first: HalfLayer {
                            pieces: new_first_half,
                            num_pieces: num_pieces - new_second_half.num_pieces,
                        },
                        second: new_second_half,
                    };

                    rotations.push((new_layer, n));
                }
            }
        }
    }

    /// Join two layers into a single bit pattern. It's assumed that the top and bottom parts
    /// compose a valid position.
    pub fn join(top: Self, bottom: Self) -> u64 {
        let mut bits = top.first.pieces;
        bits <<= 4 * top.second.num_pieces;
        bits |= top.second.pieces;
        bits <<= 4 * bottom.first.num_pieces;
        bits |= bottom.first.pieces;
        bits <<= 4 * bottom.second.num_pieces;
        bits |= bottom.second.pieces;
        bits
    }

    /// From a bit pattern, try to extract the right-most half-layer.
    ///
    /// This returns `None` if it's not possible to do it because a big piece would need to be
    /// split into two half-layers.
    fn extract_right_most_half_layer(bits: u64) -> Option<(HalfLayer, u64)> {
        // Check if the pattern can finish with a half-layer by counting how many big pieces there
        // are in the right-most positions. A half-layer can be made of either:
        // - 3 big pieces
        // - 2 big and 2 small
        // - 1 big and 4 small
        // - 6 small pieces
        // Each piece is 4 bits, and the mask 0b0010 reveals whether it's a big piece.
        const MASK_LAST_3_SIZES: u64 = 0x222;
        const MASK_LAST_4_SIZES: u64 = 0x2222;
        const MASK_LAST_5_SIZES: u64 = 0x2_2222;
        const MASK_LAST_6_SIZES: u64 = 0x22_2222;

        let num_pieces = if bits & MASK_LAST_3_SIZES == MASK_LAST_3_SIZES {
            3
        } else if (bits & MASK_LAST_4_SIZES).count_ones() == 2 {
            4
        } else if (bits & MASK_LAST_5_SIZES).count_ones() == 1 {
            5
        } else if bits & MASK_LAST_6_SIZES == 0 {
            6
        } else {
            return None;
        };

        let remaining_bits = bits >> (4 * num_pieces);
        let pieces = bits ^ (remaining_bits << (4 * num_pieces));
        Some((HalfLayer { pieces, num_pieces }, remaining_bits))
    }
}

impl fmt::Display for HalfLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in 0..self.num_pieces {
            if n > 0 {
                f.write_char(' ')?;
            }

            let shift = 4 * (self.num_pieces - n - 1);
            let piece = Piece::from_bits((self.pieces >> shift) & 0xF);
            write!(f, "{}", piece)?;
        }

        Ok(())
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn from_pieces(pieces: &[Piece]) -> Layer {
        let mut bits = 0;
        for piece in pieces {
            bits <<= 4;
            bits |= piece.as_bits();
        }

        let (second, first) = Layer::extract_right_most_half_layer(bits).unwrap();
        Layer {
            first: HalfLayer {
                pieces: first,
                num_pieces: pieces.len() as u8 - second.num_pieces,
            },
            second,
        }
    }

    #[test]
    fn rotations() {
        let pieces = &[
            Piece::WhiteRedBlue,
            Piece::WhiteBlue,
            Piece::WhiteBlueOrange,
            Piece::WhiteOrange,
            Piece::WhiteOrangeGreen,
            Piece::WhiteGreen,
            Piece::WhiteGreenRed,
            Piece::WhiteRed,
        ];
        let layer = from_pieces(pieces);

        let mut rotations = vec![];
        layer.rotations(&mut rotations);

        let expected_rotations = (0..8)
            .map(|n| {
                let mut rotated = vec![];
                rotated.extend_from_slice(&pieces[pieces.len() - n..]);
                rotated.extend_from_slice(&pieces[..pieces.len() - n]);
                (from_pieces(&rotated), n as u8)
            })
            .collect_vec();

        assert_eq!(rotations.len(), expected_rotations.len());
        for (actual, expected) in rotations.into_iter().zip(expected_rotations) {
            assert_eq!(actual, expected, "{} != {}", actual.0, expected.0);
        }
    }
}
