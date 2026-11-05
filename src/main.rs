mod backend;
mod cli;

use crate::{backend::Backend, cli::Cli};

use clap::Parser;

fn main() {
    let cli = Cli::parse();
    println!("Input: {:?}", cli.url);

    println!("browser: {:?}", cli.browser);
    println!("Match: {:?}", cli.match_patterns);
    println!("New Window Flag: {:?}", cli.new_window_flag);
    println!("Selected Backend in CLI: {:?}", cli.backend);

    let backend = cli.backend.resolve();
    println!("Detected Backend: {:?}", backend);
}
