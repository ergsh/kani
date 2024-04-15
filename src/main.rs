use anyhow::{Context, Result};

mod cli;
mod tui;

fn main() -> Result<()> {
    cli::cli()?;

    Ok(())
}
