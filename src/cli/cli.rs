use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{fs, path::Path};

use crate::common::types::{self};
use crate::tui;

const KANI_CONFIG_FILENAME: &str = "Kani.toml";
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
    // Loads config and starts the TUI app
    Start,
    // Initialize Kani.toml cofig file
    Init { name: String },
    // Add a new board under the current Kani.toml workspace
    New { name: String },
}

pub fn cli() -> Result<()> {
    // get CLI arguments
    let args = Cli::parse();

    match &args.command {
        Commands::Init { name } => {
            // create initial KaniConfig struct
            // let kani_config = types::KaniConfig {
            //     name: name.to_string(),
            //     manager: None,
            //     stages: vec![
            //         "Backlog".to_string(),
            //         "Todo".to_string(),
            //         "In Progress".to_string(),
            //         "For Review".to_string(),
            //         "Done".to_string(),
            //     ],
            //     boards: vec![
            //         types::KaniBoard {
            //             name: "Board 1 hotdog".to_string(),
            //             cards: vec![
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //             ], // end of cards[]
            //         },
            //         types::KaniBoard {
            //             name: "Board 1 hotdog".to_string(),
            //             cards: vec![
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //             ], // end of cards[]
            //         },
            //         types::KaniBoard {
            //             name: "Board 1 hotdog".to_string(),
            //             cards: vec![
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //                 types::KaniCard {
            //                     name: "Create CI CD pipeline with circle".to_string(),
            //                     stage: "In Progress".to_string(),
            //                     owners: vec![
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                         types::KaniMember {
            //                             name: "Kristian Quirapas".to_string(),
            //                             email: None,
            //                             role: None,
            //                         },
            //                     ], // end of owners[]
            //                 },
            //             ], // end of cards[]
            //         },
            //     ], // end of boards[]
            //     archive: vec![],
            // };
            // // serialize to string
            // let kani_config = toml::to_string_pretty(&kani_config)?;
            // // create Kani.toml file and write to it
            // fs::write(Path::new(KANI_CONFIG_FILENAME), kani_config)?;
            //
            // println!("{}: Created Kani.toml", "INIT".bold().green());
        }
        Commands::New { name } => {
            // read Kani.toml config file
            let kani_config = fs::read_to_string(KANI_CONFIG_FILENAME)?;
            // deserialize to struct
            let mut kani_config: types::KaniConfig = toml::from_str(&kani_config)?;
            // Add new board
            kani_config.boards.push();
        }
        Commands::Config => todo!(),
        Commands::Start => tui::app::app()?,
    }

    Ok(())
}
