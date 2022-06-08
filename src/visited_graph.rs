use crate::position::Movement;
use crate::{MovementChange, Position};
use petgraph::prelude::*;
use petgraph::visit::{VisitMap, Visitable};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VisitedGraph {
    graph: DiGraph<VisitedPosition, MovementChange>,
    position_indexes: BTreeMap<u64, NodeIndex>,
    start: NodeIndex,
    end: NodeIndex,
    main_length: u32,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Hash)]
pub struct VisitedPosition {
    position: Position,
    start_depth: u32,
    end_depth: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum VisitedKind {
    New(NodeIndex),
    Ignored,
    Improved,
}

impl VisitedGraph {
    pub fn new(solution: &[Movement]) -> Self {
        let main_length = solution.len() as u32 - 1;

        let mut graph = DiGraph::default();
        let mut position_indexes = BTreeMap::new();

        let mut start = None;
        let mut parent = None;
        for (i, &movement) in solution.iter().enumerate() {
            let idx = graph.add_node(VisitedPosition::new(
                movement.position,
                i as u32,
                main_length - i as u32,
            ));
            position_indexes.insert(movement.position.as_bytes(), idx);

            if let Some(parent) = parent {
                graph.add_edge(parent, idx, movement.change);
                graph.add_edge(idx, parent, movement.change.inversed());
            }
            parent = Some(idx);

            if start.is_none() {
                start = Some(idx);
            }
        }

        VisitedGraph {
            graph,
            position_indexes,
            start: start.unwrap(),
            end: parent.unwrap(),
            main_length,
        }
    }

    pub fn visit(
        &mut self,
        position: Position,
        change: MovementChange,
        parent: NodeIndex,
    ) -> VisitedKind {
        let parent_node = self.graph[parent];
        let new_node = VisitedPosition::new(
            position,
            parent_node.start_depth + 1,
            parent_node.end_depth + 1,
        );

        match self.position_indexes.entry(position.as_bytes()) {
            Entry::Vacant(entry) => {
                let idx = self.graph.add_node(new_node);
                self.graph.add_edge(parent, idx, change);
                self.graph.add_edge(idx, parent, change.inversed());
                entry.insert(idx);
                VisitedKind::New(idx)
            }
            Entry::Occupied(entry) => {
                let idx = *entry.get();
                let prev_node = &mut self.graph[idx];
                let prev_was_main = prev_node.start_depth + prev_node.end_depth == self.main_length;
                if prev_was_main
                    && (new_node.start_depth < prev_node.start_depth
                        || new_node.end_depth < prev_node.end_depth)
                {
                    println!("Improved {} -> {}", prev_node, new_node);
                    *prev_node = new_node;
                    self.graph.update_edge(parent, idx, change);
                    self.graph.update_edge(idx, parent, change.inversed());
                    self.reset_distances();
                    VisitedKind::Improved
                } else {
                    VisitedKind::Ignored
                }
            }
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeIndex> {
        self.graph.node_indices()
    }

    pub fn get(&self, node: NodeIndex) -> VisitedPosition {
        self.graph[node]
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn main_length(&self) -> u32 {
        self.main_length
    }

    fn reset_distances(&mut self) {
        let mut to_visit = VecDeque::with_capacity(self.graph.node_count());
        let mut visited = self.graph.visit_map();

        to_visit.push_back(self.start);
        visited.visit(self.start);
        while let Some(idx) = to_visit.pop_front() {
            let start_depth = self.graph[idx].start_depth;
            let mut neighbors = self.graph.neighbors(idx).detach();

            while let Some(next) = neighbors.next_node(&self.graph) {
                if visited.visit(next) {
                    self.graph[next].start_depth = start_depth + 1;
                    to_visit.push_back(next);
                }
            }
        }

        self.graph.reset_map(&mut visited);
        to_visit.push_back(self.end);
        visited.visit(self.end);
        while let Some(idx) = to_visit.pop_front() {
            let end_depth = self.graph[idx].end_depth;
            let mut neighbors = self.graph.neighbors(idx).detach();

            while let Some(next) = neighbors.next_node(&self.graph) {
                if visited.visit(next) {
                    self.graph[next].end_depth = end_depth + 1;
                    to_visit.push_back(next);
                }
            }
        }

        self.main_length = self.graph[self.start].end_depth;
    }
}

impl VisitedPosition {
    fn new(position: Position, start_depth: u32, end_depth: u32) -> Self {
        Self {
            position,
            start_depth,
            end_depth,
        }
    }

    pub fn position(self) -> Position {
        self.position
    }

    pub fn start_depth(self) -> u32 {
        self.start_depth
    }

    pub fn end_depth(self) -> u32 {
        self.end_depth
    }
}

impl fmt::Display for VisitedPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}+{})",
            self.position, self.start_depth, self.end_depth
        )
    }
}
