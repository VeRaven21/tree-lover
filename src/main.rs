use anyhow::Result;
use clap::Parser;
use std::path::{self, PathBuf};

mod errors;
mod filesystem;
mod renderer;
mod scanner;

use renderer::app::App;
use scanner::read_directory_recursively;

#[derive(Parser)]
struct Args {
    path: Option<PathBuf>,

    #[arg(short, long, default_value_t = 0)]
    print_files: u8,
}

fn main() -> Result<()> {
    if !is_impl() {
        panic!("Not implemented for windows lol")
    }

    let args = Args::parse();
    let mut path = args.path.unwrap_or(PathBuf::from("."));
    if !path.is_absolute() {
        path = path::absolute(path).unwrap();
    }
    let tree = read_directory_recursively(path)?;

    ratatui::run(|terminal| App::default().run(terminal, tree))?;

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
