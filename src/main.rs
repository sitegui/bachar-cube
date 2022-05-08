use crate::position::{Movement, MovementKind};
use dashmap::DashSet;
use itertools::Itertools;
use position::Position;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::error::Error;
use std::fmt;
use std::process::exit;
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

fn main() -> Result<(), Box<dyn Error>> {
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
    initial_position.for_each_movement(MovementKind::ALL, |pos| {
        println!("{}", pos.position);
    });

    let pool = ThreadPoolBuilder::new().num_threads(16).build()?;

    let seen_positions = DashSet::new();
    let mut positions = vec![Movement {
        position: initial_position,
        next_kind: MovementKind::ALL,
    }];
    let start = Instant::now();
    for _ in 0..15 {
        positions = explore_multi_thread(&pool, solved_position, &seen_positions, positions);
    }

    eprintln!("start.elapsed() = {:?}", start.elapsed());

    Ok(())
}

fn explore_multi_thread(
    pool: &ThreadPool,
    solved_position: Position,
    seen_positions: &DashSet<Position>,
    movements: Vec<Movement>,
) -> Vec<Movement> {
    let pool_size = pool.current_num_threads();
    println!("Explore {} movements", movements.len());

    let movements_by_thread = movements.len() / pool_size + 1;
    let task_results: Vec<Vec<Movement>> = pool.scope(|s| {
        movements
            .into_par_iter()
            .chunks(movements_by_thread)
            .map(|movements| {
                let mut new_movements = Vec::new();
                for movement in movements {
                    movement
                        .position
                        .for_each_movement(movement.next_kind, |new_moviment| {
                            if new_moviment.position == solved_position {
                                print!("SOLVED!");
                                exit(0);
                            }

                            if seen_positions.insert(new_moviment.position) {
                                new_movements.push(new_moviment);
                            }
                        });
                }
                new_movements
            })
            .collect()
    });

    let mut all_new_movements = Vec::with_capacity(task_results.iter().map(Vec::len).sum());
    for new_movements in task_results {
        all_new_movements.extend(new_movements);
    }
    all_new_movements
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
