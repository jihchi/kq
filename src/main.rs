use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut lines = String::new();
    io::stdin().read_line(&mut lines)?;
    let _nodes = kdl::parse_document(lines)?;
    Ok(())
}
