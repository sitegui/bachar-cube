use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Piece {
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
}

impl Piece {
    pub fn as_bits(self) -> u64 {
        use Piece::*;

        match self {
            WhiteRedBlue => piece_bits(1, false),
            WhiteBlue => piece_bits(2, false),
            WhiteBlueOrange => piece_bits(3, false),
            WhiteOrange => piece_bits(4, false),
            WhiteOrangeGreen => piece_bits(5, false),
            WhiteGreen => piece_bits(6, false),
            WhiteGreenRed => piece_bits(7, false),
            WhiteRed => piece_bits(0, false),
            YellowOrange => piece_bits(0, true),
            YellowOrangeBlue => piece_bits(1, true),
            YellowBlue => piece_bits(2, true),
            YellowBlueRed => piece_bits(3, true),
            YellowRed => piece_bits(4, true),
            YellowRedGreen => piece_bits(5, true),
            YellowGreen => piece_bits(6, true),
            YellowGreenOrange => piece_bits(7, true),
        }
    }

    pub fn from_bits(bits: u64) -> Self {
        use Piece::*;

        if bits == piece_bits(1, false) {
            WhiteRedBlue
        } else if bits == piece_bits(2, false) {
            WhiteBlue
        } else if bits == piece_bits(3, false) {
            WhiteBlueOrange
        } else if bits == piece_bits(4, false) {
            WhiteOrange
        } else if bits == piece_bits(5, false) {
            WhiteOrangeGreen
        } else if bits == piece_bits(6, false) {
            WhiteGreen
        } else if bits == piece_bits(7, false) {
            WhiteGreenRed
        } else if bits == piece_bits(0, false) {
            WhiteRed
        } else if bits == piece_bits(0, true) {
            YellowOrange
        } else if bits == piece_bits(1, true) {
            YellowOrangeBlue
        } else if bits == piece_bits(2, true) {
            YellowBlue
        } else if bits == piece_bits(3, true) {
            YellowBlueRed
        } else if bits == piece_bits(4, true) {
            YellowRed
        } else if bits == piece_bits(5, true) {
            YellowRedGreen
        } else if bits == piece_bits(6, true) {
            YellowGreen
        } else if bits == piece_bits(7, true) {
            YellowGreenOrange
        } else {
            unreachable!()
        }
    }

    pub fn size(self) -> u8 {
        use Piece::*;

        match self {
            WhiteRedBlue | WhiteBlueOrange | WhiteOrangeGreen | WhiteGreenRed
            | YellowOrangeBlue | YellowBlueRed | YellowRedGreen | YellowGreenOrange => 2,
            WhiteBlue | WhiteOrange | WhiteGreen | WhiteRed | YellowOrange | YellowBlue
            | YellowRed | YellowGreen => 1,
        }
    }
}

const fn piece_bits(seq: u64, yellow: bool) -> u64 {
    (seq << 1) | (yellow as u64)
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Piece::*;

        let s = match self {
            WhiteRedBlue => "WRB",
            WhiteBlue => "WB",
            WhiteBlueOrange => "WBO",
            WhiteOrange => "WO",
            WhiteOrangeGreen => "WOG",
            WhiteGreen => "WG",
            WhiteGreenRed => "WGR",
            WhiteRed => "WR",
            YellowOrange => "YO",
            YellowOrangeBlue => "YOB",
            YellowBlue => "YB",
            YellowBlueRed => "YBR",
            YellowRed => "YR",
            YellowRedGreen => "YRG",
            YellowGreen => "YG",
            YellowGreenOrange => "YGO",
        };

        f.write_str(s)
    }
}
