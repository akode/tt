use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    pub url: String,
}
