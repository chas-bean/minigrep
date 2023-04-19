use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args().skip(1);

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
