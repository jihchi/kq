use kdl::KdlDocument;
use std::io::{self, Read};

mod cli;

fn main() -> miette::Result<()> {
    let args = cli::Args::new().expect("failed to parse arguments");

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
    stdin
        .read_to_string(&mut buffer)
        .expect("failed to read content from stdin");

    let doc = buffer.parse::<KdlDocument>()?;
    let results = doc.query(query)?;

    if let Some(results) = results {
        print!("{}", results)
    }

    Ok(())
}
