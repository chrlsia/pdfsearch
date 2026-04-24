use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, num_args = 1..=4)]
    pub words: Vec<String>,

    #[arg(short, long, num_args = 1..)]
    pub dirs: Vec<String>,

    #[arg(short = 't', long)]
    pub threads: Option<usize>,
}