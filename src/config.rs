use config::{Config, ConfigError, File};
use resolve_path::PathResolveExt;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub notes: NotesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotesConfig {
    base_dir: PathBuf,
    daily_note_dir: PathBuf,
    attachments_dir: PathBuf,
    papers_dir: PathBuf,
}

impl NotesConfig {
    pub fn base_path(&self) -> PathBuf {
        self.base_dir.resolve().to_path_buf()
    }

    pub fn daily_note_path(&self) -> PathBuf {
        self.base_path().join(&self.daily_note_dir)
    }

    pub fn obsidian_attachments_path(&self) -> PathBuf {
        self.base_path().join(&self.attachments_dir)
    }

    pub fn obsidian_papers_path(&self) -> PathBuf {
        self.base_path().join(&self.papers_dir)
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("notes.base_dir", "~/notes")?
            .set_default("notes.daily_note_dir", "daily")?
            .set_default("notes.attachments_dir", "attachments")?
            .set_default("notes.papers_dir", "papers")?
            .add_source(File::with_name("/Users/akode/.config/tt").required(false)) // TODO: Use 'home' carate for home dir?
            .build()?;
        config.try_deserialize()
    }
}
