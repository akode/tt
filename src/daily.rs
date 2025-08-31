use askama::Template;
use chrono;
use std::fs::File;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "daily.md")]
struct DailyTemplate<'a> {
    date: &'a str,
}

pub fn create_daily() {
    let today = chrono::Utc::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let daily = DailyTemplate { date: &today };
    let path = PathBuf::from(format!("{today}.md", today = today));
    let mut file = File::create_new(path).expect("Unable to create daily note file");
    daily
        .write_into(&mut file)
        .expect("Unable to write to stdout");
}
