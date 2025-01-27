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
    pub fn extend(&mut self, rhs: Self) {
        self.extensions.extend(rhs.extensions);
        self.attributes.extend(rhs.attributes);
    }
}
