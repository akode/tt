use crate::PaperInfo;
use askama::Template;

#[derive(Template)]
#[template(path = "paper.md")]
pub struct PaperTemplate<'a> {
    title: &'a str,
    authors: &'a Vec<String>,
    summary: &'a str,
    pdf_path: &'a str,
    pdf_link: String,
    year: &'a Option<u32>,
}

impl<'a> PaperTemplate<'a> {
    pub fn new(paper_info: &'a PaperInfo, pdf_path: &'a str) -> Self {
        PaperTemplate {
            title: &paper_info.title,
            authors: &paper_info.authors,
            summary: &paper_info.summary,
            pdf_path: pdf_path,
            pdf_link: paper_info.pdf_link.to_string(),
            year: &paper_info.year,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let paper = PaperTemplate {
            title: "Paper Title",
            pdf_path: "paper.pdf",
            pdf_link: "https://www.example.com/paper.pdf".to_string(),
            summary: "Summary of the paper.",
            authors: &vec!["Peter".to_string(), "Paul".to_string()],
            year: &Some(2019),
        };
        println!("{}", paper.render().unwrap());
        assert!(true);
    }
}
