use resolve_path::PathResolveExt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub obsidian_vault_path: PathBuf,
    obsidian_attachments_dir: PathBuf,
    obsidian_papers_dir: PathBuf,
}

impl Config {
    fn obsidian_vault_path(&self) -> PathBuf {
        self.obsidian_vault_path.resolve().to_path_buf()
    }

    pub fn obsidian_attachments_path(&self) -> PathBuf {
        self.obsidian_vault_path()
            .join(&self.obsidian_attachments_dir)
    }

    pub fn obsidian_attachments_dir(&self) -> PathBuf {
        self.obsidian_attachments_dir.to_owned()
    }

    pub fn obsidian_papers_path(&self) -> PathBuf {
        self.obsidian_vault_path().join(&self.obsidian_papers_dir)
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            obsidian_vault_path: PathBuf::from("~/Documents/obs-notes"),
            obsidian_attachments_dir: PathBuf::from("attachments"),
            obsidian_papers_dir: PathBuf::from("papers"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_config() {
        let config = Config::default();
        assert_eq!(
            config.obsidian_vault_path,
            PathBuf::from("~/Documents/obs-notes")
        );
    }

    #[test]
    fn obsidian_attachments_path() {
        let config = Config::default();
        assert_eq!(
            config.obsidian_attachments_path(),
            PathBuf::from("~/Documents/obs-notes/attachments".resolve())
        );
    }
}
