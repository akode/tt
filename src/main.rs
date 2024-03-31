mod arxiv;
mod cli;
mod config;
mod errors;
mod markdown;
mod paper_info;

use std::{fs::File, io::BufWriter, path::PathBuf};

use anyhow::Result;
use arxiv::ArxivPaper;
use askama::Template;
use clap::Parser;
use config::Config;
use confy;
use markdown::PaperTemplate;
use paper_info::PaperInfo;

use url::Url;

fn fetch_file(url: &Url, file_path: &PathBuf) -> std::io::Result<u64> {
    let mut file_data = ureq::get(&url.to_string())
        .call()
        .expect("unable to fetch pdf")
        .into_reader();

    let writer = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(writer);
    std::io::copy(&mut file_data, &mut writer)
}

fn main() -> Result<()> {
    let config: Config = confy::load("tt", None)?;
    println!(
        "{}: {}",
        confy::get_configuration_file_path("tt", None)?.display(),
        config.obsidian_vault_path.display()
    );
    let cli = cli::Cli::parse();

    let url = Url::parse(&cli.url).unwrap();
    let paper_info: PaperInfo = match url.host_str() {
        Some("arxiv.org") => ArxivPaper::from_url(&url)
            .expect("Can't parse arxiv url")
            .into(),
        _ => anyhow::bail!("Unsupported host"),
    };
    // println!("{:?}", paper_info);
    let pdf_path = config
        .obsidian_attachments_path()
        .join(paper_info.pdf_file_name());
    let annotation_path = config
        .obsidian_attachments_dir()
        .join(paper_info.pdf_file_name());
    if !pdf_path.exists() {
        fetch_file(&url, &pdf_path)?;
    }
    let template = PaperTemplate::new(&paper_info, annotation_path.to_str().unwrap());
    let markdown = template.render().unwrap();
    println!("{}", markdown);

    let markdown_path = config
        .obsidian_papers_path()
        .join(paper_info.md_file_name());
    let markdown_path: &str = match markdown_path.to_str() {
        None => anyhow::bail!("Can't create path"),
        Some(p) => p,
    };
    println!("Writing markdown to {}", markdown_path);
    let writer = File::create(markdown_path).unwrap();
    let mut writer = BufWriter::new(writer);
    template.write_into(&mut writer)?;

    let _ = fetch_file(&paper_info.pdf_link, &pdf_path);

    Ok(())
}
