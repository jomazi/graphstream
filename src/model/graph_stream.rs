use super::super::{GSError, GSResult};
use super::{Edge, NodeStore, NodeType, Task, TaskSelection};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

//'a lifetime indicates that reference exists as long as the belonging GraphStream object exists
// generic type means that node identifier can be of different type, e.g. numeric ID, string literal
pub struct GraphStream<'a, T>
where
    T: PartialEq + Copy + Debug,
{
    pub edges: Vec<Edge<T>>, // go for vector instead of BTree, because multiple edges might occur at same time
    pub edge_type: &'a str,
    pub directed: bool, // whether or not edges are directed,
    pub active_tasks: Option<TaskSelection>,
    pub nodes: NodeStore<T>,
}

impl<'a, T> GraphStream<'a, T>
where
    T: PartialEq + Copy + Eq + Hash + Debug,
{
    pub fn new(edge_type: &'a str, directed: bool, tasks: Option<TaskSelection>) -> Self {
        Self {
            edges: Vec::<Edge<T>>::new(),
            edge_type: edge_type,
            directed: directed,
            active_tasks: tasks,
            nodes: NodeStore::<T>::new(),
        }
    }
    // checks if given task is active
    fn check_for_task(&self, task: Task) -> bool {
        match &self.active_tasks {
            None => false,
            Some(t) => t.get(&task).is_some(),
        }
    }
    pub fn insert(&mut self, timestamp: i64, node_1: T, node_2: T) {
        let n1: NodeType<T> = self.nodes.get_node(node_1);
        let n2: NodeType<T> = self.nodes.get_node(node_2);
        self.edges.push(Edge::new(timestamp, n1, n2));
        // task specific functionality
        if self.check_for_task(Task::Neighbors) {
            self.nodes.add_neighbor(node_1, node_2);
            self.nodes.add_neighbor(node_2, node_1)
        }
    }
    pub fn len(&self) -> usize {
        self.edges.len()
    }
    pub fn get_neighbors(&self, node_id: T) -> GSResult<HashSet<T>> {
        if !&self.check_for_task(Task::Neighbors) {
            return Err(GSError::Internal(
                "Neighbors task is not active.".to_string(),
            ));
        } else {
            match self.nodes.get_neighbors(node_id) {
                None => Ok(HashSet::<T>::new()),
                Some(n) => Ok(n),
            }
        }
    }
    // fn get_indexes(&self, node_id: T, edge_side: EdgeSide) -> Option<Vec<usize>> {
    //     match edge_side {
    //         EdgeSide::Start => {
    //             let indexes: Vec<usize> = self
    //                 .start_nodes
    //                 .iter()
    //                 .enumerate()
    //                 .filter(|_x| _x.1 == &node_id)
    //                 .map(|_x| _x.0)
    //                 .collect();
    //             if indexes.len() == 0 {
    //                 None
    //             } else {
    //                 Some(indexes)
    //             }
    //         }
    //         EdgeSide::End => {
    //             let indexes: Vec<usize> = self
    //                 .end_nodes
    //                 .iter()
    //                 .enumerate()
    //                 .filter(|_x| _x.1 == &node_id)
    //                 .map(|_x| _x.0)
    //                 .collect();
    //             if indexes.len() == 0 {
    //                 None
    //             } else {
    //                 Some(indexes)
    //             }
    //         }
    //     }
    // }
    // // method to get all adjacent nodes of given node identified by ID
    // // edge_side defines whether to only take the neighbors of this side,
    // // e.g. edge_side = Start -> given node (node_id) must appear as End node
    // // used as alternative to "neighbors" task
    // pub fn get_neighbors(&self, node_id: T, edge_side: Option<EdgeSide>) -> Option<Vec<T>> {
    //     match edge_side {
    //         Some(es) => match es {
    //             EdgeSide::Start => {
    //                 let indexes: Option<Vec<usize>> = self.get_indexes(node_id, EdgeSide::End);
    //                 match indexes {
    //                     None => None,
    //                     Some(idxs) => Some(idxs.iter().map(|i| self.start_nodes[*i]).collect()),
    //                 }
    //             }
    //             EdgeSide::End => {
    //                 let indexes: Option<Vec<usize>> = self.get_indexes(node_id, EdgeSide::Start);
    //                 match indexes {
    //                     None => None,
    //                     Some(idxs) => Some(idxs.iter().map(|i| self.end_nodes[*i]).collect()),
    //                 }
    //             }
    //         },
    //         None => {
    //             // solve recursively
    //             let mut neighbors: Vec<T> = self
    //                 .get_neighbors(node_id, Some(EdgeSide::Start))
    //                 .unwrap_or(Vec::<T>::new()); // start neighbors
    //             neighbors.append(
    //                 &mut self
    //                     .get_neighbors(node_id, Some(EdgeSide::End))
    //                     .unwrap_or(Vec::<T>::new()),
    //             ); // append end neighbors
    //             if neighbors.len() == 0 {
    //                 None
    //             } else {
    //                 Some(neighbors)
    //             }
    //         }
    //     }
    // }
}
