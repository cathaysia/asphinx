use lazy_static::lazy_static;
use minijinja::Environment;
use std::collections::btree_map::BTreeMap;
use tracing::*;
use walkdir::WalkDir;

lazy_static! {
    static ref TMPLS: BTreeMap<String, String> = {
        let mut tmpls: BTreeMap<String, String> = Default::default();
        WalkDir::new("layouts")
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|entry| match entry.path().to_str() {
                Some(v) => {
                    if v.ends_with("html.jinja") {
                        return Some(v.to_string());
                    }
                    None
                }
                None => None,
            })
            .for_each(|file_name| {
                trace!("读取文件：{}", file_name);
                let content = std::fs::read_to_string(&file_name).unwrap();
                let tpl_name = file_name.replace("layouts/", "").replace(".html.jinja", "");
                trace!("添加模板：{}", tpl_name);
                tmpls.insert(tpl_name, content);
            });
        tmpls
    };
    static ref ENGINE: Environment<'static> = {
        let mut engine = Environment::new();
        TMPLS.iter().for_each(|(ref path, ref content)| {
            engine.add_template(path, content).unwrap();
        });
        engine
    };
}

#[derive(Debug)]
pub struct Tmpl {}

impl Tmpl {
    pub fn get_engine() -> &'static Environment<'static> {
        &ENGINE
    }
}
