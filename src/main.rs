use anyhow::{Context, Result};

mod cli;
mod common;
mod tui;

fn main() -> Result<()> {
    cli::cli::cli()?;

    Ok(())
}
