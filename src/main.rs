mod find_solution;
mod piece;
mod position;
mod prefix_set;
mod rotatable_layer;
mod scorable_layer;

use crate::piece::Piece;
use crate::position::Position;
use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::ThreadPoolBuilder;
use std::time::Instant;

const NUM_THREADS: usize = 16;

fn main() -> Result<()> {
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

    println!("{}", Position::solved());
    println!("{}", initial_position);

    ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build_global()?;

    let start = Instant::now();
    let solution = find_solution::find_solution(initial_position, 10_000, NUM_THREADS)
        .context("expected a solution to be found")?;
    println!("find_solution in {:?}", start.elapsed());

    println!("{}", solution.iter().map(|m| m.change()).format(", "));

    Ok(())
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
