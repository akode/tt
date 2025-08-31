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
    Daily {},
}
