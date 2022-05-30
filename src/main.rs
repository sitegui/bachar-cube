use crate::position::{Movement, MovementChange, MovementKind};
use crate::prefix_set::PrefixSet;
use crossbeam_utils::atomic::AtomicCell;
use itertools::Itertools;
use outer_layer::OuterLayer;
use outer_piece::OuterPiece;
use parking_lot::Mutex;
use position::Position;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

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
    let solution = explore(initial_position, 1_000_000, NUM_THREADS);
    println!("start.elapsed() = {:?}", start.elapsed());

    if let Some(solution) = solution {
        println!("{:?}", solution.iter().map(|m| m.change).format(", "));
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct VisitedPosition {
    movement: Movement,
    prev_index: Option<u32>,
}

/// Maximize score, then minimize depth, then minimize index
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Enqueued {
    score: u8,
    depth: u16,
    index: u32,
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

#[derive(Debug)]
struct MainExplorer {
    iterations: usize,
    seen_positions: PrefixSet,
    visits: Vec<VisitedPosition>,
    queue: BinaryHeap<Enqueued>,
    is_solved: AtomicCell<bool>,
    solution: Mutex<Option<Vec<Movement>>>,
}

impl MainExplorer {
    fn new(initial_position: Position) -> Self {
        let mut seen_positions = PrefixSet::new();
        let mut all_movements = Vec::new();
        let mut queue = BinaryHeap::new();

        let initial_movement = Movement {
            position: initial_position,
            next_kind: MovementKind::ALL,
            change: MovementChange::Flip,
        };
        all_movements.push(VisitedPosition {
            prev_index: None,
            movement: initial_movement,
        });
        seen_positions.insert(initial_position.as_bytes());
        queue.push(Enqueued {
            score: initial_position.solved_score(),
            depth: 0,
            index: 0,
        });

        MainExplorer {
            iterations: 0,
            seen_positions,
            visits: all_movements,
            queue,
            is_solved: AtomicCell::new(false),
            solution: Mutex::new(None),
        }
    }

    fn explode(&mut self, num: usize) -> Vec<ThreadExplorer> {
        // Round-robin the queue
        let mut thread_queues = (0..num).map(|_| BinaryHeap::new()).collect_vec();
        let mut i = 0;
        while let Some(enqueued) = self.queue.pop() {
            thread_queues[i].push(enqueued);
            i = (i + 1) % thread_queues.len();
        }

        let main = &*self;
        thread_queues
            .into_iter()
            .enumerate()
            .map(move |(id, queue)| ThreadExplorer {
                id,
                iterations: 0,
                main,
                visits: vec![],
                queue,
            })
            .collect()
    }

    fn solution(&self) -> Option<Vec<Movement>> {
        self.solution.lock().clone()
    }
}

impl Explorer for MainExplorer {
    fn seen_positions(&self) -> &PrefixSet {
        &self.seen_positions
    }

    fn next_index(&self) -> u32 {
        self.visits.len() as u32
    }

    fn push_visit(&mut self, visit: VisitedPosition) {
        self.visits.push(visit)
    }

    fn queue_mut(&mut self) -> &mut BinaryHeap<Enqueued> {
        &mut self.queue
    }

    fn get_visit(&self, index: u32) -> VisitedPosition {
        self.visits[index as usize]
    }

    fn is_solved(&self) -> bool {
        self.is_solved.load()
    }

    fn set_solution(&self, solution: Vec<Movement>) {
        self.is_solved.store(true);
        *self.solution.lock() = Some(solution);
    }

    fn iterations_mut(&mut self) -> &mut usize {
        &mut self.iterations
    }
}

#[derive(Debug)]
struct ThreadExplorer<'a> {
    id: usize,
    iterations: usize,
    main: &'a MainExplorer,
    visits: Vec<VisitedPosition>,
    queue: BinaryHeap<Enqueued>,
}

trait Explorer {
    fn seen_positions(&self) -> &PrefixSet;
    fn next_index(&self) -> u32;
    fn push_visit(&mut self, visit: VisitedPosition);
    fn queue_mut(&mut self) -> &mut BinaryHeap<Enqueued>;
    fn get_visit(&self, index: u32) -> VisitedPosition;
    fn is_solved(&self) -> bool;
    fn set_solution(&self, solution: Vec<Movement>);
    fn iterations_mut(&mut self) -> &mut usize;

    fn enqueue(&mut self, parent: Enqueued, movement: Movement) {
        if self.seen_positions().insert(movement.position.as_bytes()) {
            let next_index = self.next_index();
            self.push_visit(VisitedPosition {
                movement,
                prev_index: Some(parent.index),
            });
            self.queue_mut().push(Enqueued {
                score: movement.position.solved_score(),
                depth: parent.depth + 1,
                index: next_index,
            });
        }
    }

    fn pop(&mut self) -> Option<(Enqueued, VisitedPosition)> {
        if self.is_solved() {
            return None;
        }

        self.queue_mut().pop().map(|enqueued| {
            *self.iterations_mut() += 1;
            (enqueued, self.get_visit(enqueued.index))
        })
    }

    fn mark_solved(&mut self, mut solution: VisitedPosition) {
        let mut movements = vec![];
        while let Some(prev_index) = solution.prev_index {
            movements.push(solution.movement);
            solution = self.get_visit(prev_index);
        }
        self.set_solution(movements);
    }
}

impl Explorer for ThreadExplorer<'_> {
    fn seen_positions(&self) -> &PrefixSet {
        &self.main.seen_positions
    }

    fn next_index(&self) -> u32 {
        (self.main.visits.len() + self.visits.len()) as u32
    }

    fn push_visit(&mut self, visit: VisitedPosition) {
        self.visits.push(visit);
    }

    fn queue_mut(&mut self) -> &mut BinaryHeap<Enqueued> {
        &mut self.queue
    }

    fn get_visit(&self, index: u32) -> VisitedPosition {
        let index = index as usize;
        if index < self.main.visits.len() {
            self.main.visits[index]
        } else {
            self.visits[index - self.main.visits.len()]
        }
    }

    fn is_solved(&self) -> bool {
        self.main.is_solved()
    }

    fn set_solution(&self, solution: Vec<Movement>) {
        self.main.set_solution(solution);
    }

    fn iterations_mut(&mut self) -> &mut usize {
        &mut self.iterations
    }
}

fn explore(
    initial_position: Position,
    warm_up: usize,
    num_threads: usize,
) -> Option<Vec<Movement>> {
    let solved_position = Position::solved();
    let mut explorer = MainExplorer::new(initial_position);

    while let Some((enqueued, next)) = explorer.pop() {
        if next.movement.position == solved_position {
            explorer.mark_solved(next);
            break;
        }

        next.movement
            .position
            .for_each_movement(next.movement.next_kind, |new_movement| {
                explorer.enqueue(enqueued, new_movement);
            });

        if explorer.iterations == warm_up {
            break;
        }
    }

    println!(
        "Warm up finished after {} positions, {} distinct seen, {} in the queue",
        format_big_int(explorer.iterations),
        format_big_int(explorer.seen_positions.len()),
        format_big_int(explorer.queue.len()),
    );

    let solution = explorer.solution();
    if solution.is_some() {
        return solution;
    }

    let thread_explorers = explorer.explode(num_threads);
    println!("Start threads");
    thread_explorers
        .into_par_iter()
        .for_each(|mut thread_explorer| {
            while let Some((enqueued, next)) = thread_explorer.pop() {
                if next.movement.position == solved_position {
                    thread_explorer.mark_solved(next);
                    break;
                }

                next.movement
                    .position
                    .for_each_movement(next.movement.next_kind, |new_movement| {
                        thread_explorer.enqueue(enqueued, new_movement);
                    });
            }
        });

    explorer.solution()
}

fn explore_thread() {}

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
