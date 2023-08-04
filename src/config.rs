mod asciidoc;

pub use asciidoc::Asciidoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub asciidoc: asciidoc::Asciidoc,
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
