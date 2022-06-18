use crate::piece::Piece;
use crate::position::{NeighboursStack, Position};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub mod piece;
pub mod position;
pub mod rotatable_layer;
pub mod scorable_layer;

struct Visited {
    depth: u16,
    parent: Position,
}

struct Enqueued {
    score: u8,
    depth: u16,
    position: Position,
    parent: Position,
}

impl Ord for Enqueued {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then_with(|| self.depth.cmp(&other.depth).reverse())
    }
}

impl PartialOrd for Enqueued {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Enqueued {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.depth == other.depth
    }
}

impl Eq for Enqueued {}

fn main() {
    let initial_position = Position::from_pieces([
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

    let mut visited_positions: HashMap<Position, Visited> = HashMap::new();
    let mut seen_positions: HashSet<Position> = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    let mut neighbours = NeighboursStack::new();

    initial_position.neighbours(&mut neighbours);
    for neighbour in neighbours.neighbours() {
        to_visit.push(Enqueued {
            score: neighbour.position().score(),
            depth: 1,
            position: neighbour.position(),
            parent: initial_position,
        });
    }

    let mut i = 0;
    let mut improvements = 0;
    while let Some(enqueued) = to_visit.pop() {
        i += 1;

        if i % 10_000 == 0 {
            println!(
                "Iteration {}: {} visited, {} queued, {} improved",
                format_big_int(i),
                format_big_int(visited_positions.len()),
                format_big_int(to_visit.len()),
                format_big_int(improvements)
            );
        }

        if enqueued.position == Position::solved() {
            println!("Solved!!!");
        }

        match visited_positions.entry(enqueued.position) {
            Entry::Occupied(mut occupied) => {
                if occupied.get().depth <= enqueued.depth {
                    // This position was already visited from a similar or better path
                    continue;
                }

                improvements += 1;
                occupied.insert(Visited {
                    depth: enqueued.depth,
                    parent: enqueued.parent,
                });
            }
            Entry::Vacant(vacant) => {
                vacant.insert(Visited {
                    depth: enqueued.depth,
                    parent: enqueued.parent,
                });
            }
        }

        enqueued.position.neighbours(&mut neighbours);
        for neighbour in neighbours.neighbours() {
            if seen_positions.insert(neighbour.position()) {
                to_visit.push(Enqueued {
                    score: neighbour.position().score(),
                    depth: enqueued.depth + 1,
                    position: neighbour.position(),
                    parent: enqueued.position,
                });
            }
        }
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
