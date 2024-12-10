use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Asciidoc {
    #[serde(default)]
    pub extensions: Vec<String>,
    pub attributes: HashMap<String, Value>,
}

impl Asciidoc {
    pub fn extend(&mut self, rhs: Self) {
        self.extensions.extend(rhs.extensions);
        self.attributes.extend(rhs.attributes);
    }
}

impl Default for Asciidoc {
    fn default() -> Self {
        let mut attributes: HashMap<String, Value> = Default::default();
        attributes.insert("icons".into(), "font".into());
        attributes.insert("toc".into(), 1.into());
        attributes.insert("experimental".into(), "".into());
        attributes.insert("source-highlighter".into(), "highlight.js".into());
        attributes.insert("plantuml-format".into(), "svg".into());
        // attributes.insert("stem".into(), "latexmath".into());

        Self {
            extensions: vec![
                "asciidoctor-mathematical".into(),
                "asciidoctor-diagram".into(),
            ],
            attributes,
        }
    }
}
