use super::super::{GSError, GSResult, GraphStream};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

// generic type T is used for node identifier

pub fn load_from_file<'a, P, T>(
    filename: P,
    separator: &'a str,
    header: bool,
    edge_type: &'a str,
    directed: bool,
) -> GSResult<GraphStream<'a, T>>
where
    P: AsRef<Path>,
    T: FromStr, // T has to be converted from string
{
    let file = File::open(filename)?;
    let mut lines_iter = BufReader::new(file).lines();

    // skip header
    if header {
        lines_iter.next();
    }

    let mut graph_stream = GraphStream::<'a, T>::new(edge_type, directed);

    for line in lines_iter {
        let l = line.unwrap();

        // split line
        let split: Vec<&str> = l.split(separator).collect();
        if split.len() != 3 {
            return Err(GSError::Internal(
                "Not enough values to unpack.".to_string(),
            ));
        }

        // extract timestamp
        let timestamp: i64 = split[0].parse()?;

        // extract nodes
        let node_1: T = match split[1].parse() {
            Ok(v) => v,
            Err(_) => return Err(GSError::Parse("Cannot parse node ID.".to_string())),
        };
        let node_2: T = match split[2].parse() {
            Ok(v) => v,
            Err(_) => return Err(GSError::Parse("Cannot parse node ID.".to_string())),
        };

        // add edge to graph stream
        graph_stream.insert(timestamp, node_1, node_2);
    }

    Ok(graph_stream)
}
