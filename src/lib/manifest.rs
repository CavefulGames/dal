use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::TargetVersion;

#[async_trait::async_trait]
pub trait WritableManifest: Send + Sized + Serialize + DeserializeOwned {
    #[inline]
    async fn from_file(path: impl Into<PathBuf> + Send) -> Result<Self> {
        let content = fs::read_to_string(path.into()).await?;

        Ok(toml::from_str(content.as_str())?)
    }

    #[inline]
    async fn write(&self, path: impl Into<PathBuf> + Send) -> Result<()> {
        fs::write(path.into(), toml::to_string(self)?).await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    output: Option<PathBuf>,
    input: Option<PathBuf>,
    file_extension: Option<String>,
    target_version: TargetVersion,
    pub minify: bool,
    // #[serde(default, deserialize_with = "crate::serde_utils::string_or_struct")]
    // generator: GeneratorParameters,
    modifiers: IndexMap<String, bool>,
    globals: IndexMap<String, bool>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            output: None,
            input: None,
            file_extension: Some("lua".to_owned()),
            target_version: TargetVersion::Lua53,
            minify: true,
            // generator: GeneratorParameters::RetainLines,
            modifiers: IndexMap::new(),
            globals: IndexMap::new(),
        }
    }
}

impl WritableManifest for Manifest {}

impl Manifest {
    // pub fn add_default_modifiers(&mut self) {
    //     for modifier_name in DEFAULT_LUAU_TO_LUA_MODIFIERS {
    //         self.insert_modifier(modifier_name.to_owned(), true);
    //     }
    //     if self.auto_optimize {
    //         for modifier_name in DEFAULT_OPTIMIZING_MODIFIERS {
    //             self.insert_modifier(modifier_name.to_owned(), true);
    //         }
    //     }
    // }

    // pub fn insert_modifier(&mut self, modifier_name: String, enabled: bool) {
    //     let enabled = if let Some(&old_enabled) = self.modifiers.get(&modifier_name) {
    //         old_enabled && enabled
    //     } else {
    //         enabled
    //     };
    //     self.modifiers.insert(modifier_name, enabled);
    // }

    // pub fn contains_rule(&self, modifier_name: String) -> bool {
    //     self.modifiers.contains_key(&modifier_name)
    // }

    // pub fn modifiers(&self) -> Result<Vec<Modifier>> {
    //     self.modifiers.iter()
    //         .filter_map(|(key, &value)| {
    //             if value {
    //                 Some(get_modifier_by_name(key.as_str()))
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }

    #[inline]
    pub fn modifiers(&self) -> &IndexMap<String, bool> {
        &self.modifiers
    }

    #[inline]
    pub fn target_version(&self) -> &TargetVersion {
        &self.target_version
    }

    // pub fn generator(&self) -> &GeneratorParameters {
    //     &self.generator
    // }

    #[inline]
    pub fn extension(&self) -> &Option<String> {
        &self.file_extension
    }

    #[inline]
    pub fn require_input(&self, replacement: Option<PathBuf>) -> Result<PathBuf> {
        replacement
            .or(self.input.clone())
            .ok_or_else(|| anyhow!("Error: 'inputs' is required but not provided."))
    }

    #[inline]
    pub fn require_output(&self, replacement: Option<PathBuf>) -> Result<PathBuf> {
        replacement
            .or(self.output.clone())
            .ok_or_else(|| anyhow!("Error: 'output' is required but not provided."))
    }
}
