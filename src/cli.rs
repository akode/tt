use clap::{Parser, Subcommand};
use url::Url;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands   
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch a paper from arXiv.org
    Arxiv {
        /// The URL to check.
        url: Url,
    }
}
