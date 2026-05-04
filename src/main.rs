mod cli;
mod config;
mod daily;
mod env;
mod errors;
mod file_handling;
mod pace;
mod papers;
mod recipes;

use anyhow::Result;
use clap::Parser;
use daily::create_daily;
use pace::Pace;
use papers::store_paper;

use crate::config::AppConfig;

fn main() -> Result<()> {
    let config = AppConfig::load().expect("Failed to load configuration");
    dbg!(config.clone());

    let cli = cli::Cli::parse();

    match cli.cmd {
        cli::Commands::Paper { url, source } => store_paper(url, source, config.notes),
        cli::Commands::Pace { pace_str } => {
            let pace = Pace::from_str(pace_str.as_str());
            println!("{}", &pace?);
            Ok(())
        }
        cli::Commands::Daily { offset } => create_daily(config.notes.daily_note_path(), offset),
        cli::Commands::Recipes {} => recipes::read(),
        cli::Commands::CopilotToken {} => env::export_copilot_token(),
        cli::Commands::SumDaily { md_file } => daily::sum_time_slots(md_file.unwrap_or({
            let today = chrono::Utc::now()
                .date_naive()
                .format("%Y-%m-%d")
                .to_string();
            config
                .notes
                .daily_note_path()
                .join(format!("{today}.md", today = today))
        })),
    }
}
