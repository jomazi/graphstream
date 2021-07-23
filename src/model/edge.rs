use super::NodeType;

// indicates the side of an edge (node_1 or node_2)
pub enum EdgeNode {
    First,
    Second,
}

pub struct Edge<T>
where
    T: PartialEq + Copy,
{
    pub timestamp: i64, // UNIX time; see https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.timestamp (accessed 19-7-21)
    pub node_1: NodeType<T>, // use Rc because multiple edges might point to the same node; RefCell for mutability
    pub node_2: NodeType<T>,
}

impl<T> Edge<T>
where
    T: PartialEq + Copy,
{
    pub fn new(timestamp: i64, node_1: NodeType<T>, node_2: NodeType<T>) -> Self {
        Self {
            timestamp: timestamp,
            node_1: node_1,
            node_2: node_2,
        }
    }
}
