use std::collections::BTreeMap;

//'a lifetime indicates that reference exists as long as the belonging GraphStream object exists
// generic type means that node identifier can be of different type, e.g. numeric ID, string literal

pub struct GraphStream<'a, T> {
    edge_stream: BTreeMap::<i64, [T; 2]>, // key is UNIX time; see https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.timestamp (accessed 19-7-21)
    pub edge_type: &'a str,
    pub directed: bool, // whether or not edges are directed
}

impl<'a, T> GraphStream<'a, T> {
    pub fn new(edge_type: &'a str, directed: bool) -> Self {
        Self {
            edge_stream: BTreeMap::<i64, [T; 2]>::new(),
            edge_type: edge_type,
            directed: directed,
        }
    }
    pub fn insert(&mut self, timestamp: i64, node_1: T, node_2: T) {
        self.edge_stream.insert(timestamp, [node_1, node_2]);
    }
}
