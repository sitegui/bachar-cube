use crate::Position;
use petgraph::graph::DefaultIx;
use petgraph::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct VisitedGraph {
    graph: UnGraph<VisitedPosition, ()>,
    position_indexes: BTreeMap<Position, DefaultIx>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Hash)]
pub struct VisitedPosition {
    pub position: Position,
    pub depth: u32,
}

impl VisitedGraph {
    pub fn new() -> Self {
        VisitedGraph {
            graph: UnGraph::new(),
            position_indexes: BTreeMap::new(),
        }
    }

    pub fn observe(&mut self, value: VisitedPosition) -> u32 {}
}
