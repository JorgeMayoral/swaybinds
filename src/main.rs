#![warn(clippy::pedantic)]

mod cli;
mod keybinds;

fn main() {
    if let Err(e) = cli::Cli::default().run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
