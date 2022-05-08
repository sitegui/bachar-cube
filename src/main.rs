use itertools::Itertools;
use position::Position;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::time::Instant;

mod position;

/// Represent the pieces that can be in the top or bottom layers
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
#[repr(u8)]
enum OuterPiece {
    // Top
    WhiteRedBlue1 = 0,
    WhiteRedBlue2 = 1,
    WhiteBlue = 2,
    WhiteBlueOrange1 = 4,
    WhiteBlueOrange2 = 5,
    WhiteOrange = 6,
    WhiteOrangeGreen1 = 8,
    WhiteOrangeGreen2 = 9,
    WhiteGreen = 10,
    WhiteGreenRed1 = 12,
    WhiteGreenRed2 = 13,
    WhiteRed = 14,
    // Bottom
    YellowOrangeBlue1 = 16,
    YellowOrangeBlue2 = 17,
    YellowBlue = 18,
    YellowBlueRed1 = 20,
    YellowBlueRed2 = 21,
    YellowRed = 22,
    YellowRedGreen1 = 24,
    YellowRedGreen2 = 25,
    YellowGreen = 26,
    YellowGreenOrange1 = 28,
    YellowGreenOrange2 = 29,
    YellowOrange = 30,

    Unknown = 255,
}

impl OuterPiece {
    fn can_split_before(self) -> bool {
        self as u8 % 2 == 0
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct OuterLayer {
    pieces: [OuterPiece; OUTER_LAYER_PIECES],
}

const OUTER_LAYER_PIECES: usize = 12;
const OUTER_LAYER_HALF_PIECES: usize = 6;

impl OuterLayer {
    fn new(pieces: [OuterPiece; 12]) -> Self {
        assert!(pieces[0].can_split_before());
        assert!(pieces[OUTER_LAYER_HALF_PIECES].can_split_before());

        OuterLayer { pieces }
    }

    fn for_each_movement(self, mut f: impl FnMut(Self)) {
        for shift in 1..OUTER_LAYER_PIECES {
            let new_cut = (shift + OUTER_LAYER_HALF_PIECES) % OUTER_LAYER_PIECES;

            if self.pieces[shift].can_split_before() && self.pieces[new_cut].can_split_before() {
                let mut new_pieces = [OuterPiece::Unknown; OUTER_LAYER_PIECES];
                new_pieces[..OUTER_LAYER_PIECES - shift].copy_from_slice(&self.pieces[shift..]);
                new_pieces[OUTER_LAYER_PIECES - shift..].copy_from_slice(&self.pieces[..shift]);
                f(OuterLayer { pieces: new_pieces });
            }
        }
    }

    fn flip(self, other: Self) -> (Self, Self) {
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
            },
            OuterLayer {
                pieces: new_other_pieces,
            },
        )
    }
}

fn main() {
    let initial_position = Position {
        top: OuterLayer::new([
            OuterPiece::YellowGreenOrange1,
            OuterPiece::YellowGreenOrange2,
            OuterPiece::WhiteRed,
            OuterPiece::WhiteRedBlue1,
            OuterPiece::WhiteRedBlue2,
            OuterPiece::WhiteBlue,
            OuterPiece::YellowBlueRed1,
            OuterPiece::YellowBlueRed2,
            OuterPiece::YellowRedGreen1,
            OuterPiece::YellowRedGreen2,
            OuterPiece::WhiteGreen,
            OuterPiece::YellowRed,
        ]),
        middle_solved: true,
        bottom: OuterLayer::new([
            OuterPiece::YellowGreen,
            OuterPiece::YellowOrange,
            OuterPiece::YellowOrangeBlue1,
            OuterPiece::YellowOrangeBlue2,
            OuterPiece::YellowBlue,
            OuterPiece::WhiteOrange,
            OuterPiece::WhiteGreenRed1,
            OuterPiece::WhiteGreenRed2,
            OuterPiece::WhiteBlueOrange1,
            OuterPiece::WhiteBlueOrange2,
            OuterPiece::WhiteOrangeGreen1,
            OuterPiece::WhiteOrangeGreen2,
        ]),
    };

    let solved_position = Position::solved();

    println!("{}", initial_position);
    initial_position.for_each_movement(|pos| {
        println!("{}", pos);
    });

    let mut seen_positions = HashSet::new();
    let mut pending_positions = VecDeque::new();
    pending_positions.push_back(initial_position);
    seen_positions.insert(initial_position);
    let mut visited_positions = 0;

    let start = Instant::now();
    while let Some(to_visit) = pending_positions.pop_front() {
        visited_positions += 1;
        to_visit.for_each_movement(|pos| {
            if seen_positions.insert(pos) {
                pending_positions.push_back(pos);
            }
        });

        if visited_positions % 1000 == 0 {
            println!(
                "visited = {}, seen = {}, pending = {}",
                visited_positions,
                seen_positions.len(),
                pending_positions.len()
            );
        }

        if visited_positions == 1_000_000 {
            break;
        }
    }

    eprintln!("start.elapsed() = {:?}", start.elapsed());
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
