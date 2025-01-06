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
    #[serde(default, rename = "crates")]
    pub crate_info: HashMap<String, CrateDependency>,
    #[serde(default)]
    pub mapping: HashMap<String, TypeMapping>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum CrateDependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        path: Option<String>,
        #[serde(default)]
        features: Option<Vec<String>>,
        #[serde(default)]
        default_features: Option<bool>,
        #[serde(default)]
        optional: Option<bool>,
    },
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum TypeMapping {
    Simple(String),
    Detailed {
        rust_type: String,
        #[serde(default = "default_true")]
        is_copy: bool,
        #[serde(default = "default_true")]
        is_params: bool,
        #[serde(default = "default_kind")]
        kind: String,
    },
}

fn default_true() -> bool {
    true
}

fn default_kind() -> String {
    "simple".to_string()
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub(crate) fn get_type_mapping(&self, ty: &Type) -> Option<&TypeMapping> {
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
