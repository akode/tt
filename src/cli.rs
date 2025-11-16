use std::path::PathBuf;

use clap::{Parser, Subcommand};
use url::Url;

use crate::papers::PaperSource;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch a paper
    Paper {
        /// The URL to check.
        url: Url,
        #[arg(long)]
        source: Option<PaperSource>,
    },
    /// Convert a pace to km/h
    Pace { pace_str: String },
    /// Create a new daily note
    Daily {
        /// Offset for creating the daily note for another day
        offset: Option<i64>,
    },
    /// Recipes
    Recipes {},
    /// Sum hours from daily note
    SumDaily {
        #[arg(long)]
        md_file: PathBuf,
    },
    /// Exports Githib Copilot token
    CopilotToken {},
}
