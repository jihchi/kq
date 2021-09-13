use kq;
use std::error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn error::Error>> {
    let query = "TODO";

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let nodes = kdl::parse_document(buffer)?;
    let nodes = kq::query_document(query, nodes)?;
    for node in nodes.iter() {
        println!("{}", node);
    }

    Ok(())
}
