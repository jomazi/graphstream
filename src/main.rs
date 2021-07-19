use graphstream as gs;

fn main() {
    gs::print_hello_world();
    println!("{}", gs::util::Error::Internal("test error".to_string()));
    let graph_stream = gs::GraphStream::new("co-occurrence", false);
    println!(
        "New graph stream with edge types {}.",
        graph_stream.edge_type
    )
}
