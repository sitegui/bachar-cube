use crate::position::{Movement, MovementChange, MovementKind};
use crate::prefix_set::PrefixSet;
use crate::priority_queue::PriorityQueue;
use itertools::Itertools;
use outer_layer::OuterLayer;
use outer_piece::OuterPiece;
use position::Position;
use std::cmp::Ordering;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::error::Error;
use std::time::Instant;

mod outer_layer;
mod outer_piece;
mod position;
mod prefix_set;
mod priority_queue;
mod visited_graph;

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

    let initial_position2 = Position {
        top: OuterLayer::new([
            OuterPiece::WhiteOrange,
            OuterPiece::YellowGreenOrange1,
            OuterPiece::YellowGreenOrange2,
            OuterPiece::YellowGreen,
            OuterPiece::YellowOrange,
            OuterPiece::WhiteGreen,
            OuterPiece::WhiteBlueOrange1,
            OuterPiece::WhiteBlueOrange2,
            OuterPiece::WhiteGreenRed1,
            OuterPiece::WhiteGreenRed2,
            OuterPiece::YellowBlueRed1,
            OuterPiece::YellowBlueRed2,
        ]),
        middle_solved: true,
        bottom: OuterLayer::new([
            OuterPiece::WhiteRed,
            OuterPiece::WhiteRedBlue1,
            OuterPiece::WhiteRedBlue2,
            OuterPiece::WhiteBlue,
            OuterPiece::WhiteOrangeGreen1,
            OuterPiece::WhiteOrangeGreen2,
            OuterPiece::YellowRed,
            OuterPiece::YellowRedGreen1,
            OuterPiece::YellowRedGreen2,
            OuterPiece::YellowOrangeBlue1,
            OuterPiece::YellowOrangeBlue2,
            OuterPiece::YellowBlue,
        ]),
    };

    println!("{}", initial_position2);
    initial_position2.for_each_movement(MovementKind::ALL, |pos| {
        println!("{}", pos.position);
    });

    // let pool = ThreadPoolBuilder::new().num_threads(16).build()?;
    // let seen_positions = DashSet::new();

    let start = Instant::now();
    explore_simple(initial_position2);
    eprintln!("start.elapsed() = {:?}", start.elapsed());

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct VisitedPosition {
    movement: Movement,
    prev_index: Option<usize>,
}

/// Maximize score, then minimize depth, then minimize index
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Enqueued {
    score: u8,
    depth: usize,
    index: usize,
}

impl Ord for Enqueued {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.depth.cmp(&other.depth).reverse())
            .then(self.index.cmp(&other.index))
    }
}

impl PartialOrd for Enqueued {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct SeenPositions {
    map: BTreeMap<Position, usize>,
    list: Vec<SeenPosition>,
}

#[derive(Debug, Clone)]
struct SeenPosition {
    parent: Option<usize>,
    estimated_depth: u32,
}

impl SeenPositions {
    fn new() -> Self {
        SeenPositions {
            map: BTreeMap::new(),
            list: Vec::new(),
        }
    }

    fn observe(&mut self, position: Position) {}
}

fn explore_simple(initial_position: Position) {
    let solved_position = Position::solved();
    let mut seen_positions = SeenPositions::new();
    let mut queue = BinaryHeap::new();

    let initial_movement = Movement {
        position: initial_position,
        next_kind: MovementKind::ALL,
        change: MovementChange::Flip,
    };
    seen_positions.insert(
        initial_position,
        SeenPosition {
            parent: None,
            estimated_depth: 0,
        },
    );
    queue.push(Enqueued {
        score: initial_position.solved_score(),
        depth: 0,
        index: 0,
    });

    let mut i = 0;
    let mut solved = None;
    while let Some(enqueued) = queue.pop() {
        let next = all_movements[enqueued.index];

        if next.movement.position == solved_position {
            println!("Solved at {:?}", enqueued);
            solved = Some(next);
            break;
        }

        next.movement
            .position
            .for_each_movement(next.movement.next_kind, |new_movement| {
                match seen_positions.entry(new_movement.position.as_bytes()) {
                    Entry::Vacant(entry) => {
                        entry.insert(todo!());
                    }
                    Entry::Occupied(entry) => {}
                }

                if seen_positions.insert(new_movement.position.as_bytes()) {
                    let next_index = all_movements.len();
                    all_movements.push(VisitedPosition {
                        movement: new_movement,
                        prev_index: Some(enqueued.index),
                    });
                    queue.push(Enqueued {
                        score: new_movement.position.solved_score(),
                        depth: enqueued.depth + 1,
                        index: next_index,
                    });
                }
            });

        i += 1;

        if i % 1_000_000 == 0 {
            println!(
                "Checked {} positions, {} distinct seen, {} in the queue, visited = {:?}",
                format_big_int(i),
                format_big_int(seen_positions.len()),
                format_big_int(queue.len()),
                enqueued
            );
        }
    }

    if let Some(mut solved) = solved {
        let mut changes = Vec::new();
        while let Some(prev_index) = solved.prev_index {
            changes.push(solved.movement.change);
            solved = all_movements[prev_index];
        }
        print!("Changes: {:?}", changes.iter().rev().format(", "));
    }
}

fn format_big_int(n: usize) -> String {
    if n < 1_000 {
        format!("{}", n)
    } else if n < 1_000_000 {
        format!("{:.1}k", n as f64 / 1e3)
    } else if n < 1_000_000_000 {
        format!("{:.1}M", n as f64 / 1e6)
    } else {
        format!("{:.1}G", n as f64 / 1e9)
    }
}
