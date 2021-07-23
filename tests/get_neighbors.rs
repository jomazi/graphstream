extern crate graphstream as gs;
use std::collections::HashSet;

#[test]
fn get_neighbors() {
    // graph stream with toy data
    let tasks: gs::TaskSelection = [gs::Task::Neighbors].iter().cloned().collect();
    let mut graph_stream = gs::GraphStream::<'_, &str>::new("co-occurrence", false, Some(tasks));

    graph_stream.insert(1604360704, "#rust", "#cargo");
    graph_stream.insert(1604360704, "#rust", "#clippy");
    graph_stream.insert(1604360704, "#c", "#rust");

    let result: HashSet<&str> = ["#c", "#cargo", "#clippy"].iter().cloned().collect();
    assert_eq!(graph_stream.get_neighbors("#rust").unwrap(), result);
}
