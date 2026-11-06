use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct State {
    #[serde(default)]
    pub official: BTreeSet<String>,
    #[serde(default)]
    pub aur: BTreeSet<String>,
    #[serde(default)]
    pub recipes: BTreeSet<String>,
}

impl State {
    pub fn path() -> PathBuf {
        ProjectDirs::from("org", "wisp", "wisp")
            .expect("project dirs")
            .config_dir()
            .join("state.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(&path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml::to_string_pretty(self)?)?;

        Ok(())
    }

    pub fn add_package(&mut self, name: &str, source: PackageSource) {
        match source {
            PackageSource::Official => self.official.insert(name.to_string()),
            PackageSource::Aur => self.aur.insert(name.to_string()),
        };
    }

    pub fn remove_package(&mut self, name: &str) {
        self.official.remove(name);
        self.aur.remove(name);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PackageSource {
    Official,
    Aur,
}

pub async fn clean_cache() -> Result<()> {
    let dir = ProjectDirs::from("org", "wisp", "wisp")
        .unwrap()
        .cache_dir()
        .to_path_buf();
    if dir.exists() {
        tokio::fs::remove_dir_all(&dir).await?;
    }

    tracing::info!("cache cleaned: {}", dir.display());

    Ok(())
}
