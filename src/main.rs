mod cli;
mod config;
mod daily;
mod errors;
mod file_handling;
mod pace;
mod papers;

use anyhow::Result;
use clap::Parser;
use config::Config;
use confy;
use daily::create_daily;
use pace::Pace;
use papers::store_paper;

fn main() -> Result<()> {
    let config: Config = confy::load("tt", None)?;
    println!(
        "{}: {}",
        confy::get_configuration_file_path("tt", None)?.display(),
        config.obsidian_vault_path.display()
    );
    let cli = cli::Cli::parse();

    //let paper_info: PaperInfo =
    match cli.cmd {
        cli::Commands::Paper { url, source } => store_paper(url, source, config)?,
        cli::Commands::Pace { pace_str } => {
            let pace = Pace::from_str(pace_str.as_str())?;
            println!("{}", pace);
        }
        cli::Commands::Daily {} => create_daily(),
    };

    Ok(())
}
