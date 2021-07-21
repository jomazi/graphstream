extern crate graphstream as gs;

#[test]
fn get_neighbors() {
    // graph stream with toy data
    let mut graph_stream = gs::GraphStream::<'_, &str>::new("co-occurrence", false);
    graph_stream.insert(1604360704, "#rust", "#cargo");
    graph_stream.insert(1604360704, "#rust", "#clippy");
    graph_stream.insert(1604360704, "#c", "#rust");

    assert_eq!(
        graph_stream.get_neighbors("#rust", None).unwrap(),
        vec!["#c", "#cargo", "#clippy"]
    );
    assert_eq!(
        graph_stream
            .get_neighbors("#rust", Some(gs::EdgeSide::End))
            .unwrap(),
        vec!["#cargo", "#clippy"]
    );
    assert_eq!(
        graph_stream
            .get_neighbors("#rust", Some(gs::EdgeSide::Start))
            .unwrap(),
        vec!["#c"]
    );
}
