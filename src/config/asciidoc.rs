use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asciidoc {
    pub extensions: Vec<String>,
    pub attributes: HashMap<String, Value>,
}

impl Default for Asciidoc {
    fn default() -> Self {
        let mut attributes: HashMap<String, Value> = Default::default();
        attributes.insert("icons".into(), "font".into());
        attributes.insert("toc".into(), 1.into());
        attributes.insert("experimental".into(), "".into());

        Self {
            extensions: vec![
                "asciidoctor-mathematical".into(),
                "asciidoctor-diagram".into(),
            ],
            attributes,
        }
    }
}
