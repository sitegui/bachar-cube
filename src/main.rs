use itertools::Itertools;
use position::Position;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::time::Instant;

mod position;

/// Represent the pieces that can be in the top or bottom layers
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
enum OuterPieceName {
    // Top
    WhiteRedBlue,
    WhiteBlue,
    WhiteBlueOrange,
    WhiteOrange,
    WhiteOrangeGreen,
    WhiteGreen,
    WhiteGreenRed,
    WhiteRed,
    // Bottom
    YellowOrange,
    YellowOrangeBlue,
    YellowBlue,
    YellowBlueRed,
    YellowRed,
    YellowRedGreen,
    YellowGreen,
    YellowGreenOrange,

    Empty,
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
struct OuterPiece {
    name: OuterPieceName,
    size: u8,
}

#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub enum MiddleLayer {
    Solved,
    Inverted,
}

/// Invariants:
/// - the sum of the first `num_pieces` sizes is 12
/// - the sum of the first `movable_pieces` sizes is 6
/// - no repeated element
#[derive(Debug, Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
pub struct OuterLayer {
    /// A layer is made of at least 6 pieces (6 big) or 10 pieces (2 big, 8 small).
    pieces: [OuterPiece; OUTER_LAYER_MAX_PIECES],
    num_pieces: usize,
    movable_pieces: usize,
}

const OUTER_LAYER_MAX_PIECES: usize = 10;
const OUTER_LAYER_TOTAL_SIZE: u8 = 12;
const OUTER_LAYER_HALF_SIZE: u8 = 6;

impl OuterPiece {
    const WHITE_RED_BLUE: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteRedBlue,
        size: 2,
    };
    const WHITE_BLUE: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteBlue,
        size: 1,
    };
    const WHITE_BLUE_ORANGE: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteBlueOrange,
        size: 2,
    };
    const WHITE_ORANGE: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteOrange,
        size: 1,
    };
    const WHITE_ORANGE_GREEN: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteOrangeGreen,
        size: 2,
    };
    const WHITE_GREEN: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteGreen,
        size: 1,
    };
    const WHITE_GREEN_RED: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteGreenRed,
        size: 2,
    };
    const WHITE_RED: OuterPiece = OuterPiece {
        name: OuterPieceName::WhiteRed,
        size: 1,
    };
    const YELLOW_ORANGE: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowOrange,
        size: 1,
    };
    const YELLOW_ORANGE_BLUE: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowOrangeBlue,
        size: 2,
    };
    const YELLOW_BLUE: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowBlue,
        size: 1,
    };
    const YELLOW_BLUE_RED: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowBlueRed,
        size: 2,
    };
    const YELLOW_RED: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowRed,
        size: 1,
    };
    const YELLOW_RED_GREEN: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowRedGreen,
        size: 2,
    };
    const YELLOW_GREEN: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowGreen,
        size: 1,
    };
    const YELLOW_GREEN_ORANGE: OuterPiece = OuterPiece {
        name: OuterPieceName::YellowGreenOrange,
        size: 2,
    };
    const EMPTY: OuterPiece = OuterPiece {
        name: OuterPieceName::Empty,
        size: 0,
    };
}

impl MiddleLayer {
    fn flip(self) -> MiddleLayer {
        match self {
            MiddleLayer::Solved => MiddleLayer::Inverted,
            MiddleLayer::Inverted => MiddleLayer::Solved,
        }
    }
}

impl OuterLayer {
    fn new(pieces: &[OuterPiece]) -> Self {
        assert!(pieces.len() <= OUTER_LAYER_MAX_PIECES);

        let mut movable_pieces = None;
        let mut total_size = 0;
        for (i, &piece) in pieces.iter().enumerate() {
            if total_size == OUTER_LAYER_HALF_SIZE {
                movable_pieces = Some(i);
            }
            total_size += piece.size;
        }

        assert_eq!(total_size, OUTER_LAYER_TOTAL_SIZE);

        let mut array_pieces = [OuterPiece::EMPTY; OUTER_LAYER_MAX_PIECES];
        array_pieces[..pieces.len()].copy_from_slice(pieces);

        OuterLayer {
            pieces: array_pieces,
            num_pieces: pieces.len(),
            movable_pieces: movable_pieces.unwrap(),
        }
    }

    fn for_each_movement(self, mut f: impl FnMut(Self)) {
        for shift in 1..self.num_pieces {
            let mut i = shift;
            let mut size = 0;
            let mut new_movable_pieces = 0;
            loop {
                size += self.pieces[i].size;
                new_movable_pieces += 1;

                match size.cmp(&OUTER_LAYER_HALF_SIZE) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        let mut new_pieces = [OuterPiece::EMPTY; OUTER_LAYER_MAX_PIECES];
                        new_pieces[..self.num_pieces - shift]
                            .copy_from_slice(&self.pieces[shift..self.num_pieces]);
                        new_pieces[self.num_pieces - shift..self.num_pieces]
                            .copy_from_slice(&self.pieces[..shift]);
                        f(OuterLayer {
                            pieces: new_pieces,
                            num_pieces: self.num_pieces,
                            movable_pieces: new_movable_pieces,
                        });
                    }
                    Ordering::Greater => {
                        break;
                    }
                }

                i = (i + 1) % self.num_pieces;
            }
        }
    }

    fn flip(self, other: Self) -> (Self, Self) {
        let mut new_self_pieces = [OuterPiece::EMPTY; OUTER_LAYER_MAX_PIECES];
        let mut new_other_pieces = [OuterPiece::EMPTY; OUTER_LAYER_MAX_PIECES];

        // Movable pieces
        new_self_pieces[..other.movable_pieces]
            .copy_from_slice(&other.pieces[..other.movable_pieces]);
        new_other_pieces[..self.movable_pieces]
            .copy_from_slice(&self.pieces[..self.movable_pieces]);

        // Fixed pieces
        let self_fixed = self.num_pieces - self.movable_pieces;
        let other_fixed = other.num_pieces - other.movable_pieces;
        new_self_pieces[other.movable_pieces..][..self_fixed]
            .copy_from_slice(&self.pieces[self.movable_pieces..self.num_pieces]);
        new_other_pieces[self.movable_pieces..][..other_fixed]
            .copy_from_slice(&other.pieces[other.movable_pieces..other.num_pieces]);

        (
            OuterLayer {
                pieces: new_self_pieces,
                num_pieces: other.movable_pieces + self_fixed,
                movable_pieces: other.movable_pieces,
            },
            OuterLayer {
                pieces: new_other_pieces,
                num_pieces: self.movable_pieces + other_fixed,
                movable_pieces: self.movable_pieces,
            },
        )
    }
}

fn main() {
    let initial_position = Position {
        top: OuterLayer::new(&[
            OuterPiece::YELLOW_GREEN_ORANGE,
            OuterPiece::WHITE_RED,
            OuterPiece::WHITE_RED_BLUE,
            OuterPiece::WHITE_BLUE,
            OuterPiece::YELLOW_BLUE_RED,
            OuterPiece::YELLOW_RED_GREEN,
            OuterPiece::WHITE_GREEN,
            OuterPiece::YELLOW_RED,
        ]),
        middle: true,
        bottom: OuterLayer::new(&[
            OuterPiece::YELLOW_GREEN,
            OuterPiece::YELLOW_ORANGE,
            OuterPiece::YELLOW_ORANGE_BLUE,
            OuterPiece::YELLOW_BLUE,
            OuterPiece::WHITE_ORANGE,
            OuterPiece::WHITE_GREEN_RED,
            OuterPiece::WHITE_BLUE_ORANGE,
            OuterPiece::WHITE_ORANGE_GREEN,
        ]),
    };

    let solved_position = Position::solved();

    println!("{}", initial_position);
    println!("{}", solved_position);

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

impl fmt::Display for MiddleLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MiddleLayer::Solved => {
                    "ok"
                }
                MiddleLayer::Inverted => {
                    "inv"
                }
            }
        )
    }
}

impl fmt::Display for OuterLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}|{}",
            self.pieces[..self.movable_pieces].iter().format(","),
            self.pieces[..self.num_pieces][self.movable_pieces..]
                .iter()
                .format(",")
        )
    }
}

impl fmt::Display for OuterPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for OuterPieceName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OuterPieceName::WhiteRedBlue => {
                    "WRB"
                }
                OuterPieceName::WhiteBlue => {
                    "WB"
                }
                OuterPieceName::WhiteBlueOrange => {
                    "WBO"
                }
                OuterPieceName::WhiteOrange => {
                    "WO"
                }
                OuterPieceName::WhiteOrangeGreen => {
                    "WOG"
                }
                OuterPieceName::WhiteGreen => {
                    "WG"
                }
                OuterPieceName::WhiteGreenRed => {
                    "WGR"
                }
                OuterPieceName::WhiteRed => {
                    "WR"
                }
                OuterPieceName::YellowOrange => {
                    "YO"
                }
                OuterPieceName::YellowOrangeBlue => {
                    "YOB"
                }
                OuterPieceName::YellowBlue => {
                    "YB"
                }
                OuterPieceName::YellowBlueRed => {
                    "YBR"
                }
                OuterPieceName::YellowRed => {
                    "YR"
                }
                OuterPieceName::YellowRedGreen => {
                    "YRG"
                }
                OuterPieceName::YellowGreen => {
                    "YG"
                }
                OuterPieceName::YellowGreenOrange => {
                    "YGO"
                }
                OuterPieceName::Empty => {
                    "*"
                }
            }
        )
    }
}
