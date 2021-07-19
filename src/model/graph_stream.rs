use super::node::NodeStorage;
use std::collections::BTreeMap;

//'stream lifetime indicates that reference exists as long as the belonging GraphStream object exists

pub struct GraphStream<'stream> {
    edge_stream: BTreeMap<i64, [usize; 2]>, // key is UNIX time; see https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.timestamp (accessed 19-7-21)
    pub edge_type: &'stream str,
    nodes: NodeStorage<'stream>, // used to store mapping of unique node identifiers (string) and their respective IDs
    pub directed: bool,          // whether or not edges are directed
}

impl<'stream> GraphStream<'stream> {
    pub fn new(edge_type: &'static str, directed: bool) -> Self {
        Self {
            edge_stream: BTreeMap::<i64, [usize; 2]>::new(),
            edge_type: edge_type,
            nodes: NodeStorage::<'stream>::new(),
            directed: directed,
        }
    }
    pub fn insert(&mut self, timestamp: i64, node_1: &'stream str, node_2: &'stream str) {
        self.edge_stream.insert(
            timestamp,
            [
                self.nodes.get_id_or_insert(node_1),
                self.nodes.get_id_or_insert(node_2),
            ],
        );
    }
}
