use minijinja::{value::Value, Error, ErrorKind, State};

pub fn minify(_state: &State, value: &Value) -> Result<Value, Error> {
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
