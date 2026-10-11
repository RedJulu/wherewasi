use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WWIData {
    pub notes: Vec<WWINote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WWISettings {
    pub show_info: bool,
}

impl Default for WWISettings {
    fn default() -> Self {
        Self { show_info: true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WWINote {
    pub id: u32,
    pub text: String,
    pub created: DateTime<Utc>,
    #[serde(alias = "sticky", deserialize_with = "deserialize_manual", default)]
    pub manual: bool,
    pub dismissed: bool,
}

fn deserialize_manual<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StickyOrManual {
        Bool(bool),
        String(String),
        Int(i64),
    }

    let val = Option::<StickyOrManual>::deserialize(deserializer)?;
    match val {
        Some(StickyOrManual::Bool(b)) => Ok(!b),
        Some(StickyOrManual::String(s)) => Ok(s != "true"),
        Some(StickyOrManual::Int(n)) => Ok(n == 0),
        None => Ok(false),
    }
}

fn notes_path() -> Result<PathBuf> {
    let current = std::env::current_dir().context("Could not find current directory")?;
    Ok(current.join(".wherewasi"))
}

fn config_path() -> Result<PathBuf> {
    let config = dirs::config_dir().context("Could not find config directory")?;
    Ok(config.join("wherewasi").join("config.json"))
}

pub fn load_data() -> Result<WWIData> {
    let path = notes_path()?;

    if !path.exists() {
        return Ok(WWIData::default());
    }

    let contents =
        fs::read_to_string(&path).with_context(|| format!("Unable to read {}", path.display()))?;

    serde_json::from_str(&contents)
        .with_context(|| format!("Unable to parse JSON from {}", path.display()))
}

pub fn save_data(data: &WWIData) -> Result<()> {
    let path = notes_path()?;

    if data.notes.is_empty() {
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Unable to delete {}", path.display()))?;
        }

        return Ok(());
    }

    let json = serde_json::to_string_pretty(data).context("Failed to serialize data")?;

    fs::write(&path, json).with_context(|| format!("Unable to write {}", path.display()))
}

pub fn load_settings() -> Result<WWISettings> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(WWISettings::default());
    }

    let contents =
        fs::read_to_string(&path).with_context(|| format!("Unable to read {}", path.display()))?;

    serde_json::from_str(&contents)
        .with_context(|| format!("Unable to parse JSON from {}", path.display()))
}

pub fn save_settings(settings: &WWISettings) -> Result<()> {
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Unable to create {}", parent.display()))?;
    }

    let json = serde_json::to_string_pretty(settings).context("Failed to serialize settings")?;

    fs::write(&path, json).with_context(|| format!("Unable to write {}", path.display()))
}
