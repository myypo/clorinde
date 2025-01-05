use miette::Diagnostic;
use postgres_types::Type;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub types: Types,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Types {
    #[serde(default, rename = "crate")]
    pub crate_info: Option<CrateInfo>,
    #[serde(default)]
    pub mapping: HashMap<String, TypeMapping>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CrateInfo {
    /// Name of the custom types crate
    #[serde(default = "default_name")]
    pub name: String,
    /// Path of the custom types crate (relative to Clorinde)
    #[serde(default = "default_path")]
    pub path: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TypeMapping {
    pub rust_type: String,
    #[serde(default = "default_true")]
    pub is_copy: bool,
    #[serde(default = "default_true")]
    pub is_params: bool,
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_true() -> bool {
    true
}

fn default_kind() -> String {
    "simple".to_string()
}

fn default_name() -> String {
    "ctypes".to_string()
}

fn default_path() -> String {
    "../ctypes".to_string()
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn get_type_mapping(&self, ty: &Type) -> Option<&TypeMapping> {
        let key = format!("{}.{}", ty.schema(), ty.name());
        self.types.mapping.get(&key)
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
}
