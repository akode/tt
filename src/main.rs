mod arxiv;
mod cli;
mod config;
mod errors;
mod file_handling;
mod markdown;
mod paper_info;

use std::{fs::File, io::BufWriter};

use anyhow::Result;
use arxiv::ArxivPaper;
use askama::Template;
use clap::Parser;
use config::Config;
use confy;
use file_handling::fetch_file;
use markdown::PaperTemplate;
use paper_info::PaperInfo;


fn main() -> Result<()> {
    let config: Config = confy::load("tt", None)?;
    println!(
        "{}: {}",
        confy::get_configuration_file_path("tt", None)?.display(),
        config.obsidian_vault_path.display()
    );
    let cli = cli::Cli::parse();

    let paper_info: PaperInfo = match cli.cmd {
        cli::Commands::Arxiv{url} => ArxivPaper::from_url(&url).unwrap().into(),
    };

    let pdf_path = config
        .obsidian_attachments_path()
        .join(paper_info.pdf_file_name());
    let annotation_path = config
        .obsidian_attachments_dir()
        .join(paper_info.pdf_file_name());

    let template = PaperTemplate::new(&paper_info, annotation_path.to_str().unwrap());
    let markdown = template.render().unwrap();
    println!("{}", markdown);

    // Write markdown file
    let markdown_path = config
        .obsidian_papers_path()
        .join(paper_info.md_file_name());
    let markdown_path: &str = match markdown_path.to_str() {
        None => anyhow::bail!("Can't create path"),
        Some(p) => p,
    };
    println!("Writing markdown file to {}", markdown_path);
    let writer = File::create(markdown_path).unwrap();
    let mut writer = BufWriter::new(writer);
    template.write_into(&mut writer)?;

    // Download PDF
    let _ = fetch_file(&paper_info.pdf_link, &pdf_path);

    Ok(())
}
