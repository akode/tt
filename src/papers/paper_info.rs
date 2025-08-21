use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use regex::Regex;
use scraper;
use ureq;
use url::Url;

use super::arxiv::ArxivPaper;

#[derive(Debug)]
pub struct PaperInfo {
    pub title: String,
    pub authors: Vec<String>,
    pub summary: String,
    pub year: Option<u32>,
    pub pdf_link: Url,
}

impl PaperInfo {
    pub fn pdf_file_name(&self) -> PathBuf {
        let sanitized_title = &self.sanitized_title().replace(" ", "_");
        match self.year {
            Some(year) => PathBuf::from(format!("{}_{}.pdf", year, sanitized_title)),
            None => PathBuf::from(format!("{}.pdf", sanitized_title)),
        }
    }

    pub fn md_file_name(&self) -> PathBuf {
        Path::new(&self.sanitized_title()).with_extension("md")
    }

    fn sanitized_title(&self) -> String {
        let re = Regex::new(r"[ :]+").unwrap();
        re.replace_all(&self.title, " ").to_string()
    }
}

impl From<ArxivPaper> for PaperInfo {
    fn from(paper: ArxivPaper) -> Self {
        get_arxiv_paper(&paper.id).expect("Failed to get paper info")
    }
}

/// Creates a PaperInfo from arxiv url.
///
/// There are two cases:
/// 1. The url is a pdf url, e.g. https://arxiv.org/pdf/1802.09639.pdf
/// 2. The url is a abstract url, e.g. https://arxiv.org/abs/1802.09639
fn get_arxiv_paper(arxiv_id: &str) -> Result<PaperInfo> {
    let abs_url = Url::parse(&format!("https://arxiv.org/abs/{}", arxiv_id)).unwrap();
    let pdf_link = Url::parse(&format!("https://arxiv.org/pdf/{}.pdf", arxiv_id)).unwrap();
    let html = ureq::get(abs_url.as_str())
        .call()
        .context("Unable to fetch page")?
        .into_string()?;
    let doc = scraper::Html::parse_document(&html);
    let title_selector = scraper::Selector::parse("meta[name='citation_title']").unwrap();
    let authors_selector = scraper::Selector::parse("meta[name='citation_author']").unwrap();
    let summary_selector = scraper::Selector::parse("meta[name='citation_abstract']").unwrap();
    let year_selector = scraper::Selector::parse("meta[name='citation_date']").unwrap();
    let title = doc
        .select(&title_selector)
        .next()
        .unwrap()
        .attr("content")
        .unwrap()
        .to_string();
    let authors = doc
        .select(&authors_selector)
        .map(|node| node.attr("content").unwrap().to_string())
        .collect();
    let summary = doc
        .select(&summary_selector)
        .next()
        .unwrap()
        .attr("content")
        .unwrap()
        .to_string();
    let year: Option<u32> = doc
        .select(&year_selector)
        .next()
        .unwrap()
        .attr("content")
        .unwrap()
        .split("/")
        .next()
        .unwrap()
        .parse::<u32>()
        .ok();
    Ok(PaperInfo {
        title,
        authors,
        summary,
        year,
        pdf_link,
    })
}
