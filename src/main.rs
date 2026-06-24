use anyhow::Result;
use std::path::Path;

mod errors;
mod node;
mod scanner;
mod renderer;

use scanner::read_directory_recursively;

fn main() -> Result<()> {
    let path = Path::new(".");

    let tree = read_directory_recursively(path)?;

    tree.draw();

    Ok(())
}
