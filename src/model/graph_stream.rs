//'a lifetime indicates that reference exists as long as the belonging GraphStream object exists
// generic type means that node identifier can be of different type, e.g. numeric ID, string literal
pub struct GraphStream<'a, T>
where
    T: PartialEq + Copy,
{
    timestamps: Vec<i64>, // UNIX time; see https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.timestamp (accessed 19-7-21)
    start_nodes: Vec<T>,
    end_nodes: Vec<T>,
    pub edge_type: &'a str,
    pub directed: bool, // whether or not edges are directed
}

pub enum EdgeSide {
    Start,
    End,
}

impl<'a, T> GraphStream<'a, T>
where
    T: PartialEq + Copy,
{
    pub fn new(edge_type: &'a str, directed: bool) -> Self {
        Self {
            timestamps: Vec::<i64>::new(),
            start_nodes: Vec::<T>::new(),
            end_nodes: Vec::<T>::new(),
            edge_type: edge_type,
            directed: directed,
        }
    }
    pub fn insert(&mut self, timestamp: i64, start_node: T, end_node: T) {
        self.timestamps.push(timestamp);
        self.start_nodes.push(start_node);
        self.end_nodes.push(end_node);
    }
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }
    fn get_indexes(&self, node_id: T, edge_side: EdgeSide) -> Option<Vec<usize>> {
        match edge_side {
            EdgeSide::Start => {
                let indexes: Vec<usize> = self
                    .start_nodes
                    .iter()
                    .enumerate()
                    .filter(|_x| _x.1 == &node_id)
                    .map(|_x| _x.0)
                    .collect();
                if indexes.len() == 0 {
                    None
                } else {
                    Some(indexes)
                }
            }
            EdgeSide::End => {
                let indexes: Vec<usize> = self
                    .end_nodes
                    .iter()
                    .enumerate()
                    .filter(|_x| _x.1 == &node_id)
                    .map(|_x| _x.0)
                    .collect();
                if indexes.len() == 0 {
                    None
                } else {
                    Some(indexes)
                }
            }
        }
    }
    // method to get all adjacent nodes of given node identified by ID
    // edge_side defines whether to only take the enighbors of this side,
    // e.g. edge_side = Start -> given node (node_id) must appear as End node
    pub fn get_neighbors(&self, node_id: T, edge_side: Option<EdgeSide>) -> Option<Vec<T>> {
        match edge_side {
            Some(es) => match es {
                EdgeSide::Start => {
                    let indexes: Option<Vec<usize>> = self.get_indexes(node_id, EdgeSide::End);
                    match indexes {
                        None => None,
                        Some(idxs) => Some(idxs.iter().map(|i| self.start_nodes[*i]).collect()),
                    }
                }
                EdgeSide::End => {
                    let indexes: Option<Vec<usize>> = self.get_indexes(node_id, EdgeSide::Start);
                    match indexes {
                        None => None,
                        Some(idxs) => Some(idxs.iter().map(|i| self.end_nodes[*i]).collect()),
                    }
                }
            },
            None => {
                // solve recursively
                let mut neighbors: Vec<T> = self
                    .get_neighbors(node_id, Some(EdgeSide::Start))
                    .unwrap_or(Vec::<T>::new()); // start neighbors
                neighbors.append(
                    &mut self
                        .get_neighbors(node_id, Some(EdgeSide::End))
                        .unwrap_or(Vec::<T>::new()),
                ); // append end neighbors
                if neighbors.len() == 0 {
                    None
                } else {
                    Some(neighbors)
                }
            }
        }
    }
}
