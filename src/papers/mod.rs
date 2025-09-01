mod arxiv;
mod markdown;
mod paper_info;

use crate::{config::Config, file_handling::fetch_file};
use anyhow::Result;
use arxiv::ArxivPaper;
use askama::Template;
use markdown::PaperTemplate;
use paper_info::PaperInfo;
use std::{fs::File, io::BufWriter};
use url::Url;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PaperSource {
    Arxiv,
}

fn determine_paper_source(url: &Url) -> Option<PaperSource> {
    match url.domain() {
        Some("www.arxiv.org") => Some(PaperSource::Arxiv),
        _ => None,
    }
}

pub fn store_paper(url: Url, source: Option<PaperSource>, config: Config) -> Result<()> {
    let paper_source: Option<PaperSource> = match (source, determine_paper_source(&url)) {
        (Some(source), _) => Some(source),
        (None, Some(source)) => Some(source),
        (None, None) => None,
    };

    let paper_info: Option<PaperInfo> = match paper_source {
        Some(PaperSource::Arxiv) => Some(ArxivPaper::from_url(&url).unwrap().into()),
        _ => None,
    };

    if let Some(paper_info) = paper_info {
        let pdf_path = config
            .obsidian_attachments_path()
            .join(paper_info.pdf_file_name());
        let annotation_path = config
            .obsidian_attachments_dir()
            .join(paper_info.pdf_file_name());

        let template = PaperTemplate::new(&paper_info, annotation_path.to_str().unwrap());
        // let markdown = template.render().unwrap();

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
    }

    Ok(())
}
