use getopts::{Fail, Matches, Options};
use std::env;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

pub struct Args {
    opts: Options,
    matches: Matches,
    program: String,
}

impl Args {
    pub fn new() -> Result<Args, Fail> {
        let args: Vec<String> = env::args().collect();
        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optflag("v", "version", "print the version");
        let matches = opts.parse(&args[1..])?;

        Ok(Args {
            opts,
            matches,
            program: args[0].clone(),
        })
    }

    pub fn help(&self) -> bool {
        self.matches.opt_present("h")
    }

    pub fn version(&self) -> bool {
        self.matches.opt_present("v")
    }

    pub fn get_free(&self) -> Option<String> {
        let some = !self.matches.free.is_empty();

        some.then(|| self.matches.free[0].clone())
    }

    pub fn print_help(&self) {
        let brief = format!("Usage: {} [options] <selector>", self.program);
        print!("{}", self.opts.usage(&brief));
    }

    pub fn print_version(&self) {
        println!("{}", VERSION.unwrap_or(""));
    }
}
