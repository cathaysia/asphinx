use lazy_static::lazy_static;
use minijinja::{Environment, Source};

use crate::jinjaext;

lazy_static! {
    static ref ENGINE: Tmpl<'static> = Tmpl::new();
}

#[derive(Debug)]
pub struct Tmpl<'a> {
    engine: Environment<'a>,
}

impl Tmpl<'_> {
    pub fn new() -> Self {
        let mut engine = Environment::new();
        engine.set_source(Source::with_loader(|name| {
            let file_name = format!("layouts/{}.html.jinja", name);
            match std::fs::read_to_string(file_name) {
                Ok(v) => return Ok(Some(v)),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(err) => Err(minijinja::Error::new(
                    minijinja::ErrorKind::InvalidOperation,
                    "could not read template",
                )
                .with_source(err)),
            }
        }));

        engine.add_filter("minify", jinjaext::minify);

        let resource = jinjaext::Resource::new();
        engine.add_global("resource", minijinja::value::Value::from_object(resource));

        Self { engine }
    }
    pub fn get_engine() -> &'static Environment<'static> {
        &ENGINE.engine
    }
}
