use crate::position::{Movement, NeighboursStack};
use crate::prefix_set::PrefixSet;
use crate::{format_big_int, Position};
use crossbeam_utils::atomic::AtomicCell;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

#[derive(Debug)]
struct MainExplorer {
    iterations: usize,
    seen_positions: PrefixSet,
    visits: Vec<VisitedPosition>,
    queue: BinaryHeap<Enqueued>,
    is_solved: AtomicCell<bool>,
    solution: Mutex<Option<Vec<Movement>>>,
    rejections: usize,
}

#[derive(Debug)]
struct ThreadExplorer<'a> {
    iterations: usize,
    main: &'a MainExplorer,
    visits: Vec<VisitedPosition>,
    queue: BinaryHeap<Enqueued>,
    rejections: usize,
}

trait Explorer {
    fn insert_position(&mut self, position: Position) -> bool;
    fn next_index(&self) -> u32;
    fn push_visit(&mut self, visit: VisitedPosition);
    fn queue_mut(&mut self) -> &mut BinaryHeap<Enqueued>;
    fn get_visit(&self, index: u32) -> VisitedPosition;
    fn is_solved(&self) -> bool;
    fn set_solution(&self, solution: Vec<Movement>);
    fn iterations_mut(&mut self) -> &mut usize;
    fn rejections_mut(&mut self) -> &mut usize;

    fn enqueue(&mut self, parent: Enqueued, movement: Movement) {
        if self.insert_position(movement.position()) {
            let next_index = self.next_index();
            self.push_visit(VisitedPosition {
                movement,
                prev_index: Some(parent.index),
            });
            self.queue_mut().push(Enqueued {
                score: movement.position().score(),
                depth: parent.depth + 1,
                index: next_index,
            });
        } else {
            *self.rejections_mut() += 1;
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
        movements.push(solution.movement);
        movements.reverse();
        self.set_solution(movements);
    }
}

impl MainExplorer {
    fn new(initial_position: Position) -> Self {
        let seen_positions = PrefixSet::new();
        let mut all_movements = Vec::new();
        let mut queue = BinaryHeap::new();

        let initial_movement = Movement::initial_movement(initial_position);
        all_movements.push(VisitedPosition {
            prev_index: None,
            movement: initial_movement,
        });
        seen_positions.insert(initial_position.as_bytes());
        queue.push(Enqueued {
            score: initial_position.score(),
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
            rejections: 0,
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
            .map(move |queue| ThreadExplorer {
                iterations: 0,
                main,
                visits: vec![],
                queue,
                rejections: 0,
            })
            .collect()
    }

    fn solution(&self) -> Option<Vec<Movement>> {
        self.solution.lock().clone()
    }
}

impl Explorer for MainExplorer {
    fn insert_position(&mut self, position: Position) -> bool {
        self.seen_positions.insert(position.as_bytes())
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

    fn rejections_mut(&mut self) -> &mut usize {
        &mut self.rejections
    }
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

impl Explorer for ThreadExplorer<'_> {
    fn insert_position(&mut self, position: Position) -> bool {
        self.main.seen_positions.insert(position.as_bytes())
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

    fn rejections_mut(&mut self) -> &mut usize {
        &mut self.rejections
    }
}

pub fn find_solution(
    initial_position: Position,
    warm_up: usize,
    num_threads: usize,
) -> Option<Vec<Movement>> {
    let solved_position = Position::solved();
    let mut explorer = MainExplorer::new(initial_position);
    let mut neighbours = NeighboursStack::new();

    while let Some((enqueued, next)) = explorer.pop() {
        if next.movement.position() == solved_position {
            explorer.mark_solved(next);
            break;
        }

        next.movement.position().neighbours(&mut neighbours);
        for &new_movement in neighbours.neighbours() {
            explorer.enqueue(enqueued, new_movement);
        }

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
            let mut neighbours = NeighboursStack::new();

            while let Some((enqueued, next)) = thread_explorer.pop() {
                if next.movement.position() == solved_position {
                    thread_explorer.mark_solved(next);

                    println!(
                        "Solved after {} iterations, {} distinct seen, {} in the queue, {} rejections",
                        format_big_int(thread_explorer.iterations),
                        format_big_int(thread_explorer.main.seen_positions.len()),
                        format_big_int(thread_explorer.queue.len()),
                        format_big_int(thread_explorer.rejections)
                    );

                    break;
                }

                next.movement.position().neighbours(&mut neighbours);
                for &new_movement in neighbours.neighbours() {
                    thread_explorer.enqueue(enqueued, new_movement);
                }
            }
        });

    explorer.solution()
}
