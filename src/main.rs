use lazy_static::lazy_static;
use minijinja::Environment;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::path;
use tokio::{fs, process};
use tracing::*;
use tracing_subscriber;

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    title: String,
    body: String,
}

fn get_body_of_html(html: &str) -> Option<String> {
    let ht = Html::parse_document(html);
    let selector = Selector::parse("body").unwrap();
    for body in ht.select(&selector) {
        return Some(body.inner_html().trim().into());
    }
    None
}

fn get_title_of_html(html: &str) -> Option<String> {
    let ht = Html::parse_document(html);
    let selector = Selector::parse("title").unwrap();
    for body in ht.select(&selector) {
        return Some(body.inner_html().trim().into());
    }
    None
}

async fn generate_html(file_path: String) {
    let file_cwd: String;

    {
        let file_path = path::Path::new(&file_path);
        if !file_path.exists() {
            warn!("路径 {} 不存在", file_path.display());
            return;
        }
        if !file_path.is_file() {
            warn!("路径 {} 指向的不是一个文件，忽略", file_path.display());
            return;
        }
        file_cwd = file_path.parent().unwrap().to_str().unwrap().into();
    }

    let des_dir = file_cwd.replace("content", "public");
    if let Err(err) = std::fs::create_dir_all(&des_dir) {
        error!("创建 {} 时发生错误：{}", des_dir, err);
        return;
    }
    let file_des_path = file_path
        .replace("content", "public")
        .replace(".adoc", ".html");

    info!("生成文件：{} -> {}", file_path, file_des_path);
    let output = process::Command::new("asciidoctor")
        .arg(file_path)
        .arg("-D")
        .arg(&des_dir)
        .arg("-o")
        .arg("-")
        .output()
        .await
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    let data = Document {
        title: get_title_of_html(&output).unwrap(),
        body: get_body_of_html(&output).unwrap(),
    };

    let mut env = Environment::new();
    env.add_template("single", include_str!("../layouts/single.html.jinja"))
        .unwrap();
    let tmpl = env.get_template("single").unwrap();
    let ctx = minijinja::value::Value::from_serializable(&data);
    let res = tmpl.render(ctx).unwrap();

    if let Err(err) = fs::write(file_des_path, &res).await {
        eprintln!("写入文件失败：{}", err);
    }
}

fn handle_file(file_path_str: String) -> Vec<String> {
    let mut result = Vec::<String>::new();
    debug!("处理文件：{}", file_path_str);

    let file_path = path::Path::new(&file_path_str);
    if !file_path.exists() {
        warn!("文件 {} 不存在", file_path.display());
        return result;
    }

    if file_path.ends_with("index.adoc") {
        let dir_path = file_path.parent().unwrap();
        let content = std::fs::read_to_string(&file_path_str).unwrap();

        let re = Regex::new(r"xref:(.*)\[.*\]").unwrap();
        for item in re.captures_iter(&content) {
            let file_name: String = item.get(1).unwrap().as_str().into();
            let file_path: String = dir_path.join(file_name.as_str()).to_str().unwrap().into();
            result.append(&mut handle_file(file_path));
        }
    }

    result.push(file_path_str);

    return result;
}

fn main() {
    RUNTIME.block_on(async {
        // TODO: 使用沙盒限制程序能够读取的路径

        tracing_subscriber::fmt::init();
        let file_path = "content/index.adoc";

        let mut files = handle_file(file_path.into());
        let b = files.iter_mut().map(|item| generate_html(item.to_string()));

        futures::future::join_all(b).await;
    });
}
