use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum XboxButton {
    A,
    B,
    X,
    Y,
    LB,
    RB,
    LT,
    RT,
    LS,
    RS,
    Start,
    Select,

    #[serde(alias = "DPADUP")]
    DPadUp,
    #[serde(alias = "DPADDOWN")]
    DPadDown,
    #[serde(alias = "DPADLEFT")]
    DPadLeft,
    #[serde(alias = "DPADRIGHT")]
    DPadRight,

    LSUp,
    LSDown,
    LSLeft,
    LSRight,
    RSUp,
    RSDown,
    RSLeft,
    RSRight,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profile {
    pub name: String,
    pub mappings: HashMap<evdev::Key, XboxButton>,
}

pub fn load_profiles() -> Result<Vec<Profile>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Err(anyhow!(
            "Configuration file not found. Create one at: {}",
            config_path.display()
        ));
    }

    let config_str = fs::read_to_string(&config_path).with_context(|| {
        format!(
            "Failed to read configuration file: {}",
            config_path.display()
        )
    })?;

    let profiles: Vec<Profile> = serde_json::from_str(&config_str)
        .with_context(|| "Failed to parse JSON from configuration file")?;

    Ok(profiles)
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("Could not find user configuration directory"))?;
    let app_config_dir = config_dir.join("xbkbremap");

    fs::create_dir_all(&app_config_dir)?;

    Ok(app_config_dir.join("config.json"))
}
