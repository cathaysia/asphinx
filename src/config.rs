mod asciidoc;

pub use asciidoc::Asciidoc;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub asciidoc: asciidoc::Asciidoc,
}

impl Config {
    pub async fn from_file(path: impl AsRef<std::path::Path>) -> Self {
        match fs::read_to_string(path).await {
            Ok(config) => match toml::from_str(&config) {
                Ok(config) => config,
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }
}

#[test]
fn test_deser() {
    let data = r#"
[asciidoc]
extensions = ["asciidoctor-mathematical", "asciidoctor-diagram"]

[asciidoc.attributes]
icons = "font"
    "#;
    let res: Config = toml::from_str(data).unwrap();

    assert_eq!(
        "font",
        res.asciidoc
            .attributes
            .get("icons")
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        &["asciidoctor-mathematical", "asciidoctor-diagram"],
        res.asciidoc.extensions.as_slice()
    );
}
