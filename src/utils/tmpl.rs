use minijinja::Environment;

use super::jinjaext::{self, LocalTime};

#[derive(Debug)]
pub struct Tmpl {
    pub engine: Box<Environment<'static>>,
}

impl Tmpl {
    pub fn new(layout_dir: String) -> Self {
        let mut engine = Box::new(Environment::new());
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

        engine.add_filter("minify", jinjaext::minify);

        let resource = jinjaext::Resource::new();
        engine.add_global("resource", minijinja::value::Value::from_object(resource));
        engine.add_global(
            "now",
            minijinja::value::Value::from_object(LocalTime::default()),
        );

        Self { engine }
    }
}
