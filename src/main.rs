use minigrep::{self, Config};
use std::env;
use std::process;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let a: Vec<_> = env::args().collect();
    println!("{a:?}");
}
