use crate::errors::ExtractionError;
use anyhow::Result;
use url::Url;

pub struct ArxivPaper {
    pub id: String,
}

impl ArxivPaper {
    pub fn from_url(url: &Url) -> Result<Self> {
        let mut path_segments = url
            .path_segments()
            .expect("Failed to extract url path segments");
        let arxiv_id = match path_segments.next() {
            Some("pdf") => path_segments
                .next()
                .expect("Failed to extract arxiv id")
                .split(".")
                .collect::<Vec<_>>()
                .split_last()
                .expect("Unable to remove extension")
                .1
                .join("."),
            Some("abs") => path_segments
                .next()
                .expect("Failed to extract arxiv id")
                .to_owned(),
            _ => return Err(ExtractionError::UnknownDomain(url.to_string()).into()),
        };
        Ok(Self { id: arxiv_id })
    }
}
