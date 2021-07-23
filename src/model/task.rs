use std::collections::HashSet;

// tasks that might be executed on graph stream, e.g. keeping track of node centralities or neighbors
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Task {
    Neighbors,
    Centrality,
}

pub type TaskSelection = HashSet<Task>;
