use crate::outer_layer::OUTER_LAYER_PIECES;
use std::fmt;

/// Represent the pieces that can be in the top or bottom layers. A large piece is in fact
/// represented by two pieces.
///
/// It's internally represented by an u8 whose bits are organized like this:
/// ```text
/// 0 0 s s s s w c
/// ```
///
/// The least significant bit `c` is 1 when the piece can be cut before. The second half of a large
/// piece will have this flag set to 0.
///
/// The bit `w` indicates if this piece is in the white (top) layer. Yellow pieces will have this
/// flag set to 0.
///
/// The four `s` bits represent the "sequence" of the piece in their layer. This is a value from
/// 0 (0b000) to 11 (0b1011).
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum OuterPiece {
    // Top
    WhiteRedBlue1 = piece_tag(0, true, true),
    WhiteRedBlue2 = piece_tag(1, true, false),
    WhiteBlue = piece_tag(2, true, true),
    WhiteBlueOrange1 = piece_tag(3, true, true),
    WhiteBlueOrange2 = piece_tag(4, true, false),
    WhiteOrange = piece_tag(5, true, true),
    WhiteOrangeGreen1 = piece_tag(6, true, true),
    WhiteOrangeGreen2 = piece_tag(7, true, false),
    WhiteGreen = piece_tag(8, true, true),
    WhiteGreenRed1 = piece_tag(9, true, true),
    WhiteGreenRed2 = piece_tag(10, true, false),
    WhiteRed = piece_tag(11, true, true),
    // Bottom
    YellowOrangeBlue1 = piece_tag(0, false, true),
    YellowOrangeBlue2 = piece_tag(1, false, false),
    YellowBlue = piece_tag(2, false, true),
    YellowBlueRed1 = piece_tag(3, false, true),
    YellowBlueRed2 = piece_tag(4, false, false),
    YellowRed = piece_tag(5, false, true),
    YellowRedGreen1 = piece_tag(6, false, true),
    YellowRedGreen2 = piece_tag(7, false, false),
    YellowGreen = piece_tag(8, false, true),
    YellowGreenOrange1 = piece_tag(9, false, true),
    YellowGreenOrange2 = piece_tag(10, false, false),
    YellowOrange = piece_tag(11, false, true),

    Unknown = 255,
}

const fn piece_tag(sequence: u8, is_white: bool, can_split_before: bool) -> u8 {
    (sequence << 2) | ((is_white as u8) << 1) | can_split_before as u8
}

impl OuterPiece {
    pub const MAX_TAG: u8 = Self::WhiteRed as u8;

    pub fn sequence(self) -> u8 {
        (self as u8) >> 2
    }

    pub fn is_white(self) -> bool {
        self as u8 & 0b10 != 0
    }

    pub fn can_split_before(self) -> bool {
        self as u8 & 0b1 != 0
    }

    /// Return the is the other piece is the successor of this one in a solved position
    pub fn is_followed_by(self, other: Self) -> bool {
        let next_sequence = (self.sequence() + 1) % OUTER_LAYER_PIECES as u8;
        self.is_white() == other.is_white() && next_sequence == other.sequence()
    }
}

impl fmt::Display for OuterPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            OuterPiece::WhiteRedBlue1 => "WRB",
            OuterPiece::WhiteRedBlue2 => "",
            OuterPiece::WhiteBlue => "WB",
            OuterPiece::WhiteBlueOrange1 => "WBO",
            OuterPiece::WhiteBlueOrange2 => "",
            OuterPiece::WhiteOrange => "WO",
            OuterPiece::WhiteOrangeGreen1 => "WOG",
            OuterPiece::WhiteOrangeGreen2 => "",
            OuterPiece::WhiteGreen => "WG",
            OuterPiece::WhiteGreenRed1 => "WGR",
            OuterPiece::WhiteGreenRed2 => "",
            OuterPiece::WhiteRed => "WR",
            OuterPiece::YellowOrange => "YO",
            OuterPiece::YellowOrangeBlue1 => "YOB",
            OuterPiece::YellowOrangeBlue2 => "",
            OuterPiece::YellowBlue => "YB",
            OuterPiece::YellowBlueRed1 => "YBR",
            OuterPiece::YellowBlueRed2 => "",
            OuterPiece::YellowRed => "YR",
            OuterPiece::YellowRedGreen1 => "YRG",
            OuterPiece::YellowRedGreen2 => "",
            OuterPiece::YellowGreen => "YG",
            OuterPiece::YellowGreenOrange1 => "YGO",
            OuterPiece::YellowGreenOrange2 => "",
            OuterPiece::Unknown => "*",
        };
        write!(f, "{}", str)
    }
}
