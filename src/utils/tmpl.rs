use minijinja::Environment;
use rust_embed::RustEmbed;

use super::jinjaext::{self, LocalTime};

#[derive(RustEmbed)]
#[folder = "builtin"]
struct Asset;

#[derive(Debug)]
pub struct Tmpl {
    pub engine: Box<Environment<'static>>,
}

impl Tmpl {
    pub fn new(layout_dir: Option<String>) -> Self {
        let mut engine = Box::new(Environment::new());
        match layout_dir {
            Some(layout_dir) => {
                engine.set_loader(move |name| {
                    let file_name = format!("{}/{}.html.jinja", layout_dir, name);
                    match std::fs::read_to_string(file_name) {
                        Ok(v) => Ok(Some(v)),
                        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
                        Err(err) => Err(minijinja::Error::new(
                            minijinja::ErrorKind::InvalidOperation,
                            "could not read template",
                        )
                        .with_source(err)),
                    }
                });
            }
            None => {
                engine.set_loader(move |name| {
                    let path = format!("layouts/{name}.html.jinja");
                    let content = String::from_utf8(
                        Asset::get(&path)
                            .unwrap_or_else(|| panic!("Cannot found {path}"))
                            .data
                            .to_vec(),
                    )
                    .unwrap();

                    Ok(Some(content))
                });
            }
        }

        engine.add_filter("minify", jinjaext::minify_jinja);

        let resource = jinjaext::Resource::new();
        engine.add_global("resource", minijinja::value::Value::from_object(resource));
        engine.add_global(
            "now",
            minijinja::value::Value::from_object(LocalTime::default()),
        );

        Self { engine }
    }
}
