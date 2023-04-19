//!
//! Pure rust implementation of the [Unified Configuration Interface](https://openwrt.org/docs/guide-user/base-system/uci)
//!

use std::collections::HashMap;

pub mod parser;
pub mod writer;

/// Main configuration group
///
/// Each configuration group has it's own file in `/etc/config`.
#[derive(Debug)]
pub struct Config {
    pub name: String,
    /// Sections with their priority order
    pub sections: Vec<Section>,
}

/// Secondary configuration group
///
/// Every [Config] is divided into sections.
#[derive(Debug)]
pub struct Section {
    pub name: Option<String>,
    pub type_: String,
    /// Either a string or a Vec of Strings
    pub options: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn to_document() -> anyhow::Result<toml_edit::Document> {
        // let mut doc = toml_edit::Document::new();

        todo!()
    }
}
