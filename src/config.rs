mod asciidoc;

use std::str::FromStr;

pub use asciidoc::Asciidoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub asciidoc: asciidoc::Asciidoc,
}

impl FromStr for Config {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = toml::from_str::<Config>(s);
        Ok(match res {
            Ok(config) => {
                let mut res = Self::default();
                res.asciidoc.extend(config.asciidoc);
                res
            }
            Err(_) => Self::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

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
