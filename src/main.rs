use crate::find_solution::find_solution;
use crate::position::{MovementChange, MovementKind};
use itertools::Itertools;
use outer_layer::OuterLayer;
use outer_piece::OuterPiece;
use position::Position;
use rayon::ThreadPoolBuilder;
use std::error::Error;
use std::time::Instant;

mod find_solution;
mod outer_layer;
mod outer_piece;
mod position;
mod prefix_set;
mod priority_queue;
mod visited_graph;

const NUM_THREADS: usize = 16;

fn main() -> Result<(), Box<dyn Error>> {
    let initial_position = Position::with_layers(
        OuterLayer::new([
            OuterPiece::YellowOrange,
            OuterPiece::WhiteGreen,
            OuterPiece::WhiteBlueOrange1,
            OuterPiece::WhiteBlueOrange2,
            OuterPiece::WhiteGreenRed1,
            OuterPiece::WhiteGreenRed2,
            OuterPiece::YellowBlueRed1,
            OuterPiece::YellowBlueRed2,
            OuterPiece::WhiteOrange,
            OuterPiece::YellowGreenOrange1,
            OuterPiece::YellowGreenOrange2,
            OuterPiece::YellowGreen,
        ]),
        true,
        OuterLayer::new([
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
    );

    println!("{}", Position::solved());
    println!("{}", initial_position);
    initial_position.for_each_movement(MovementKind::ALL, |pos| {
        println!("{}", pos.position);
    });

    ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build_global()?;

    let start = Instant::now();
    let solution = find_solution(initial_position, 1_000_000, NUM_THREADS);
    println!("start.elapsed() = {:?}", start.elapsed());

    if let Some(solution) = solution {
        println!("{:?}", solution.iter().map(|m| m.change).format(", "));
    }

    Ok(())
}

fn improve() {}

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
