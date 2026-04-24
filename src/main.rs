mod cli;
mod search;
mod highlight;
mod utils;

use clap::Parser;
use cli::Args;
use search::run_search;

fn main() {
    let args = Args::parse();

    run_search(args);
}