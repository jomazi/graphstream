extern crate graphstream as gs;

#[test]
fn load_from_file() {
    let graph_stream = gs::load_from_file::<_, u8>(
        "./tests/test-stream.csv",
        ",",
        true,
        "co-occurrence",
        false,
        None,
    )
    .unwrap();
    assert_eq!(graph_stream.len(), 3)
}
