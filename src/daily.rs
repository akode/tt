use anyhow::Result;
use askama::Template;
use markdown::mdast::{Heading, List, ListItem, Node, Paragraph, Text};
use markdown::{ParseOptions, to_mdast};
use std::fs::File;
use std::ops::Add;
use std::path::PathBuf;

use regex::Regex;

#[derive(Template)]
#[template(path = "daily.md")]
struct DailyTemplate<'a> {
    date: &'a str,
}

/// Creates a daily note based on a template file.
pub fn create_daily(offset: Option<i64>) -> Result<()> {
    let today = chrono::Utc::now()
        .date_naive()
        .add(chrono::TimeDelta::days(offset.unwrap_or(0_i64)))
        .format("%Y-%m-%d")
        .to_string();
    let daily = DailyTemplate { date: &today };
    let path = PathBuf::from(format!("{today}.md", today = today));
    let mut file = File::create_new(path).expect("Unable to create daily note file");
    daily
        .write_into(&mut file)
        .expect("Unable to write to stdout");
    Ok(())
}

/// Finds lists in daily notes markdown file and sums up the total time spent.
pub fn sum_time_slots(file_name: PathBuf) -> Result<()> {
    let path = PathBuf::from(file_name);
    let content = std::fs::read_to_string(path).expect("Unable to read daily note file");
    let mdast = to_mdast(&content, &ParseOptions::default()).expect("Unable to parse markdown");
    if let Some(nodes) = mdast.children() {
        let count = process_nodes(nodes);
        println!("{}", count);
    }
    Ok(())
}

/// Recursively processes list items to find time slots and sum their durations.
fn process_nodes(items: &Vec<Node>) -> f32 {
    items.iter().fold(0.0, |acc, node| match node {
        Node::List(List {
            children,
            position: _,
            ordered: _,
            start: _,
            spread: _,
        }) => acc + process_nodes(children),
        Node::ListItem(ListItem {
            children,
            position: _,
            spread: _,
            checked: _,
        }) => acc + process_nodes(children),
        Node::Paragraph(Paragraph {
            children,
            position: _,
        }) => acc + process_nodes(children),
        Node::Heading(Heading {
            children,
            position: _,
            depth: _,
        }) => {
            // TODO: Implement handling of the headings
            acc + process_nodes(children)
        }
        Node::Text(Text { value, position: _ }) => {
            let re =
                Regex::new(r"(?<start>[0-9]{1,2}:[0-5][0-9]) - (?<end>[0-9]{1,2}:[0-5][0-9]):")
                    .expect("Invalid regex");

            if let Some(captures) = re.captures(value) {
                let start_str = captures.name("start").unwrap().as_str();
                let end_str = captures.name("end").unwrap().as_str();

                let start_time = chrono::NaiveTime::parse_from_str(start_str, "%H:%M")
                    .expect("Invalid start time format");
                let end_time = chrono::NaiveTime::parse_from_str(end_str, "%H:%M")
                    .expect("Invalid end time format");

                let duration = end_time - start_time;
                let hours = duration.num_minutes() as f32 / 60.0;

                acc + hours
            } else {
                acc
            }
        }
        _ => acc,
    })
}
