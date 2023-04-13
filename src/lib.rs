//!
//! Pure rust implementation of the [Unified Configuration Interface](https://openwrt.org/docs/guide-user/base-system/uci)
//!

use std::collections::HashMap;

mod parser;

/// Main configuration group
///
/// Each configuration group has it's own file in `/etc/config`.
#[derive(Debug)]
pub struct Config {
    name: String,
    /// Sections with their priority order
    sections: Vec<Section>,
}

/// Secondary configuration group
///
/// Every [Config] is divided into sections.
#[derive(Debug)]
pub struct Section {
    name: Option<String>,
    type_: String,
    /// Either a string or a Vec of Strings
    options: HashMap<String, Vec<String>>,
}
