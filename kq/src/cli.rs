use getopts::{Fail, Matches, Options};
use std::env;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Args {
    opts: Options,
    matches: Matches,
    program: String,
}

impl Args {
    pub fn new() -> Result<Args, Fail> {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();
        let mut opts = Options::new();

        opts.optflag("h", "help", "print this help menu");
        opts.optflag("v", "version", "print the version");

        let matches = opts.parse(&args[1..])?;

        Ok(Args {
            opts,
            matches,
            program,
        })
    }

    pub fn help(&self) -> bool {
        self.matches.opt_present("h")
    }

    pub fn version(&self) -> bool {
        self.matches.opt_present("v")
    }

    pub fn get_query(&self) -> Option<&String> {
        self.matches.free.get(0)
    }

    pub fn print_help(&self) {
        let brief = format!("Usage: {} [options] <selector>", self.program);
        print!("{}", self.opts.usage(&brief));
    }

    pub fn print_version(&self) {
        println!("{}", CARGO_PKG_VERSION);
    }
}
