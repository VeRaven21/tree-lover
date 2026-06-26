use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod errors;
mod node;
mod renderer;
mod scanner;

use scanner::read_directory_recursively;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    print_files: u8,

    #[arg(long, default_value_t = -1)]
    depth: i64
}

fn main() -> Result<()> {
    let path = Path::new(".");
    let args = Args::parse();
    let tree = read_directory_recursively(path, args.depth)?;

    tree.draw(args.print_files > 0);

    Ok(())
}
