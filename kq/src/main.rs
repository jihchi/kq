use kdl::KdlDocument;
use std::error;
use std::io::{self, Read};
use kd::select;

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

    let selector = match args.get_query() {
        Some(selector) => selector,
        None => {
            args.print_help();
            return Ok(());
        }
    };

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let document = buffer.parse::<KdlDocument>()?;
    let nodes = select(selector, document)?;
    nodes.iter().for_each(|node| println!("{}", node));

    Ok(())
}
