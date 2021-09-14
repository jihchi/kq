use std::error;
use std::io::{self, Read};

mod cli;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = cli::Args::new()?;

    if args.help() {
        args.print_help();
        return Ok(());
    }

    if args.version() {
        args.print_version();
        return Ok(());
    }

    let query = match args.get_query() {
        Some(query) => query,
        None => {
            args.print_help();
            return Ok(());
        }
    };

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let nodes = kdl::parse_document(buffer)?;
    let nodes = kq::query_document(query, nodes)?;
    nodes.iter().for_each(|node| println!("{}", node));

    Ok(())
}
