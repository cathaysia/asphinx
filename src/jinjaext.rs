use core::fmt;
use std::io;

use minijinja::{
    value::{Object, Value},
    Error, ErrorKind, State,
};

pub(crate) fn minify_inner(value: &str) -> Result<Value, Error> {
    let mut cfg = minify_html::Cfg::new();
    cfg.minify_css = true;
    cfg.minify_js = true;
    let res = minify_html::minify(value.to_string().as_bytes(), &cfg);
    match String::from_utf8(res) {
        Ok(v) => return Ok(Value::from(v)),
        Err(err) => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("minify failed: {}", err),
            ))
        }
    }
}

pub fn minify(_state: &State, value: &Value) -> Result<Value, Error> {
    minify_inner(&value.to_string())
}

#[derive(Debug)]
pub struct Resource {}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Object for Resource {
    fn call_method(&self, _state: &State, name: &str, args: &[Value]) -> Result<Value, Error> {
        match name {
            "Get" => match Resource::get(&args.first().unwrap().to_string()) {
                Ok(v) => {
                    return Ok(Value::from(v));
                }
                Err(e) => return Err(Error::new(ErrorKind::InvalidOperation, e.to_string())),
            },
            &_ => {
                todo!()
            }
        }
    }
}

impl Resource {
    pub fn new() -> Self {
        Self {}
    }

    fn get(name: &str) -> io::Result<String> {
        std::fs::read_to_string("assets/".to_string() + name)
    }
}
