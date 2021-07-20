use graphstream as gs;

fn main() {
    gs::print_hello_world();
    println!("{}", gs::util::Error::Internal("test error".to_string()));
    let mut graph_stream = gs::GraphStream::<'_, &str>::new("co-occurrence", false);
    println!(
        "New graph stream with edge types {}.",
        graph_stream.edge_type
    );
    graph_stream.insert(1604360704, "#rust", "#cargo");
    let mut graph_stream_numeric = gs::GraphStream::<'_, u8>::new("co-occurrence", false);
    graph_stream_numeric.insert(1604360704, 1, 2);
}
