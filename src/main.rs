use anyhow::Result;
use clap::Parser;
use ratatui;
use std::path::Path;

mod errors;
mod node;
mod renderer;
mod scanner;

use renderer::app::App;
use scanner::read_directory_recursively;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    print_files: u8,

    #[arg(long, default_value_t = -1)]
    depth: i64,
}

fn main() -> Result<()> {
    if !is_impl() {
        panic!("Not implemented for windows lol")
    }

    let path = Path::new(".");
    let args = Args::parse();
    let tree = read_directory_recursively(path, args.depth)?;

    ratatui::run(|terminal| App::default().run(terminal, &tree))?;

    Ok(())
}

fn is_impl() -> bool {
    #[cfg(target_family = "unix")]
    {
        true
    }

    #[cfg(target_family = "windows")]
    {
        false
    } // Probably will test it for windows sometime in the future
}
