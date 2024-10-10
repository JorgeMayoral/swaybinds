#![warn(clippy::pedantic)]

mod cli;

fn main() {
    if let Err(e) = cli::Cli::default().run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
