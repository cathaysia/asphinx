use super::jinjaext::{self, LocalTime};
use minijinja::Environment;
use tracing::error;

#[derive(Debug)]
pub struct Tmpl {
    pub engine: Box<Environment<'static>>,
}

impl Tmpl {
    pub fn new(theme_dir: String) -> Self {
        let mut engine = Box::new(Environment::new());
        engine.set_loader(move |name| {
            let file_name = format!("{theme_dir}/layouts/{name}.html");
            match std::fs::read_to_string(&file_name) {
                Ok(v) => Ok(Some(v)),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(err) => {
                    error!("failed get {file_name}");
                    Err(minijinja::Error::new(
                        minijinja::ErrorKind::InvalidOperation,
                        "could not read template",
                    )
                    .with_source(err))
                }
            }
        });

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
