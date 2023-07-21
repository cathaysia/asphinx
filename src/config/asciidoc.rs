use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Asciidoc {
    pub extensions: Vec<String>,
    pub attributes: HashMap<String, Value>,
}
