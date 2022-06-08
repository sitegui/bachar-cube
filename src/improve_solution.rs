use crate::position::Movement;
use crate::visited_graph::{VisitedGraph, VisitedKind};
use crate::{format_big_int, MovementKind};
use petgraph::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct PendingSeed {
    visited: NodeIndex,
}

pub fn improve_solution(base_solution: &[Movement]) {
    let mut graph = VisitedGraph::new(base_solution);

    let mut to_visit = VecDeque::new();
    for visited in graph.nodes() {
        to_visit.push_back(PendingSeed { visited });
    }

    let mut iterations = 0;
    let mut total_visits = 0;
    let mut new_visits = 0;
    let mut ignored_visits = 0;
    let mut improved_visits = 0;

    while let Some(pending) = to_visit.pop_front() {
        iterations += 1;
        let visited = graph.get(pending.visited);

        visited
            .position()
            .for_each_movement(MovementKind::ALL, |movement| {
                total_visits += 1;

                let kind = graph.visit(movement.position, movement.change, pending.visited);

                match kind {
                    VisitedKind::New(idx) => {
                        new_visits += 1;
                        to_visit.push_back(PendingSeed { visited: idx });
                    }
                    VisitedKind::Ignored => {
                        ignored_visits += 1;
                    }
                    VisitedKind::Improved => {
                        improved_visits += 1;
                    }
                }
            });

        if iterations % 100_000 == 0 {
            println!("Iteration {}", format_big_int(iterations));
            println!(
                "Visits: total={}, new={}, ignored={}, improved={}",
                format_big_int(total_visits),
                format_big_int(new_visits),
                format_big_int(ignored_visits),
                format_big_int(improved_visits),
            );
            println!(
                "Graph: {} nodes, {} main length",
                format_big_int(graph.node_count()),
                graph.main_length()
            );
        }
    }
}
