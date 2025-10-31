use anyhow::Result;

pub fn export_copilot_token() -> Result<()> {
    let file = std::fs::File::open("~/.config/github-copilot/apps.json")?;
    let reader = std::io::BufReader::new(file);

    let json: serde_json::Value = serde_json::from_reader(reader)?;
    println!("{:#?}", json);

    Ok(())
}
