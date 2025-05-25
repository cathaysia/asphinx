mod asciidoc;

use std::{path::Path, str::FromStr};

pub use asciidoc::Asciidoc;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub no_default: bool,
    #[serde(default)]
    pub asciidoc: asciidoc::Asciidoc,
    #[serde(default)]
    pub site: String,
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(include_str!("../asphinx.toml")).unwrap()
    }
}

impl Config {
    pub fn merge(&mut self, patch: Self) {
        let default = Self::default();

        self.asciidoc.merge(patch.asciidoc);
        if patch.site != default.site {
            self.site = patch.site;
        }
    }

    pub async fn from_path(path: impl AsRef<Path>) -> Self {
        let mut res = Self::default();

        if let Ok(config) = fs::read_to_string(path).await {
            if let Ok(config) = toml::from_str::<Self>(&config) {
                if config.no_default {
                    return config;
                }
                res.merge(config);
            }
        }

        res
    }
}

impl FromStr for Config {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = toml::from_str::<Config>(s);
        Ok(match res {
            Ok(config) => {
                let mut res = Self::default();
                res.asciidoc.merge(config.asciidoc);
                res
            }
            Err(_) => Self::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use itertools::Itertools;

    use super::Config;

    #[test]
    fn test_deser() {
        let data = r#"
[asciidoc]
extensions = ["asciidoctor-mathematical", "asciidoctor-diagram"]

[asciidoc.attributes]
icons = "font"
    "#;
        // let res = Config::from_str(data).unwrap();
        let res: Config = toml::from_str(data).unwrap();
        let values = res
            .asciidoc
            .extensions
            .iter()
            .map(|item| item.as_str())
            .collect_vec();

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
            values.as_slice()
        );
    }

    #[test]
    fn test_from_config() {
        let config = Config::from_str(
            r#"
[asciidoc]
extensions = ["asciidoctor-mathematical", "asciidoctor-diagram"]

[asciidoc.attributes]
plantuml-format = "svg"
        "#,
        )
        .unwrap();
        println!("{config:#?}");

        assert!(config.asciidoc.attributes.contains_key("plantuml-format"));
    }
}
