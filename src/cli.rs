use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{error::Error, fs::File, path::Path};

const CURRENT_DIRECTORY: &str = ".";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Configure boards with TUI
    Config,
    // Start the TUI app
    Start,
    // Initialize Kani.toml cofig file
    Init,
    // Add a new board under the current Kani.toml workspace
    New { name: String },
}

pub fn cli() -> Result<()> {
    // get CLI arguments
    let args = Cli::parse();

    match &args.command {
        Commands::Init => {
            let _toml_file = File::create(Path::new("Kani.toml"))?;

            println!("{}: Created Kani.toml", "INIT".bold().green());

            // Generate `config.json` file
        }
        Commands::New { name } => {
            // Create board named {name}
            let _board_file = File::create(Path::new(&format!("{}.kani", name)))?;

            println!(
                "{}: Created {}",
                "NEW".bold().green(),
                format!("{}.kani", name).bold().green()
            );
        }
        Commands::Config => todo!(),
        Commands::Start => todo!(),
    }

    Ok(())
}
