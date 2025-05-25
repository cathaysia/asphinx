use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Asciidoc {
    pub extensions: Vec<String>,
    pub attributes: HashMap<String, Value>,
}

impl Asciidoc {
    pub fn merge(&mut self, patch: Self) {
        self.extensions.extend(patch.extensions);
        self.attributes.extend(patch.attributes);
    }
}
