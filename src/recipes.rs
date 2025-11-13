use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_norway;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, PartialEq, Deserialize, Debug)]
struct Recipe {
    title: String,
    ingredients: Option<Vec<Ingredient>>,
    tags: Option<Vec<String>>,
    steps: Option<Vec<String>>,
    section: String,
}

#[derive(Serialize, PartialEq, Deserialize, Debug)]
struct Ingredient {
    name: String,
    qty: Option<String>,
}

pub fn read() -> Result<()> {
    let file =
        File::open("/Users/akode/dev/notes/rezepte/rezepte.yaml").expect("Unable to read file!");
    let rdr = BufReader::new(file);

    let recipes: Vec<Recipe> = serde_norway::from_reader(rdr).expect("Unable to parse YAML!");
    println!("{:?}", recipes);
    Ok(())
}
