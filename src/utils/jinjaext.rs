use core::fmt;
use std::{io, sync::Arc};

use minijinja::{
    value::{Object, Value},
    Error, ErrorKind, State,
};
use time::{format_description, OffsetDateTime};

pub(crate) fn minify(value: &str) -> String {
    let mut cfg = minify_html::Cfg::new();
    cfg.minify_css = true;
    cfg.minify_js = true;
    let res = minify_html::minify(value.to_string().as_bytes(), &cfg);
    String::from_utf8(res).unwrap_or(value.to_string())
}

pub fn minify_jinja(_state: &State, value: &Value) -> Value {
    minify(&value.to_string()).into()
}

#[derive(Debug)]
pub struct Resource {}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Object for Resource {
    fn call_method(
        self: &Arc<Self>,
        _state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "Get" => match Resource::get(&args.first().unwrap().to_string()) {
                Ok(v) => Ok(Value::from(v)),
                Err(e) => Err(Error::new(ErrorKind::InvalidOperation, e.to_string())),
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

#[derive(Debug)]
pub struct LocalTime {
    local_time: OffsetDateTime,
}

impl fmt::Display for LocalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for LocalTime {
    fn default() -> Self {
        Self {
            local_time: OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()),
        }
    }
}

impl Object for LocalTime {
    fn call_method(
        self: &Arc<Self>,
        _state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "Format" => {
                if args.is_empty() {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::MissingArgument,
                        "需要一个参数",
                    ));
                }
                if args.len() > 1 {
                    return Err(minijinja::Error::new(
                        minijinja::ErrorKind::TooManyArguments,
                        format!("需要一个参数，但是提供了 {} 个", args.len()),
                    ));
                }
                let format_arg = args[0].to_string();
                match format_description::parse_borrowed::<1>(&format_arg) {
                    Ok(time_format) => Ok(self.local_time.format(&time_format).unwrap().into()),
                    Err(err) => Err(minijinja::Error::new(
                        minijinja::ErrorKind::InvalidOperation,
                        format!("提供的格式化参数解析失败：{}", err),
                    )),
                }
            }
            method => Err(minijinja::Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("提供的格式化参数解析失败：{}", method),
            )),
        }
    }
}
