use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path;
use tokio::{fs, process};
use tracing::*;
use tracing_subscriber;

mod duration;
mod git;
mod html;
mod tmpl;
use duration::SelfDuration;
use html::HtmlParser;
use tmpl::Tmpl;

use clap::Parser;

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    title: String,
    content: Option<String>,
    toc: Option<String>,
    footnotes: Option<String>,
    last_modify_date: Option<String>,
    build_date: String,
}

async fn move_assets(item: &str, source: &str, des: &str) {
    let source_file = path::Path::new(source).join(item);
    if !source_file.exists() {
        warn!("文件不存在：{}", source_file.display());
        return;
    }
    let des_file = path::Path::new(des).join(item);
    let des_path = des_file.parent().unwrap();
    if !des_path.exists() {
        fs::create_dir_all(des_path).await.unwrap();
    }

    info!(
        "移动文件：{} -> {}",
        source_file.display(),
        des_file.display()
    );
    fs::copy(source_file, des_file).await.unwrap();
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
        .arg(&file_path)
        .arg("-D")
        .arg(&des_dir)
        .arg("-o")
        .arg("-")
        .arg("-a")
        .arg("toc=1")
        .arg("-a")
        .arg(format!("outdir={}", &des_dir))
        .arg("-a")
        .arg("imagesdir=assets_mermaid")
        .arg("-r")
        .arg("asciidoctor-diagram")
        .output()
        .await
        .unwrap();
    let output = String::from_utf8_lossy(&output.stdout).to_string();

    let html = HtmlParser::new(&output);

    let now = chrono::Utc::now();

    let data = Document {
        title: html.get_title(),
        content: html.get_content(),
        toc: html.get_toc(),
        footnotes: html.get_footnotes(),
        last_modify_date: git::get_last_commit_time(&file_path).await,
        build_date: format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
    };

    let tmpl = Tmpl::get_engine().get_template("single").unwrap();
    let ctx = minijinja::value::Value::from_serializable(&data);
    let res = tmpl.render(ctx).unwrap();

    if let Err(err) = fs::write(&file_des_path, &res).await {
        eprintln!("写入文件失败：{}", err);
    }

    let assets = html.get_image_urls();
    let acts = assets
        .iter()
        .filter(|item| item.ends_with("_mermaid"))
        .map(|item| move_assets(&item, &file_cwd, &des_dir));
    futures::future::join_all(acts).await;
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

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t=Level::WARN)]
    level: Level,
}

fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt().with_max_level(args.level).init();

    let start_time = std::time::Instant::now();
    RUNTIME.block_on(async {
        // TODO: 使用沙盒限制程序能够读取的路径

        let file_path = "content/index.adoc";

        let mut files = handle_file(file_path.into());
        let b = files.iter_mut().map(|item| generate_html(item.to_string()));

        futures::future::join_all(b).await;
    });
    let end_time = std::time::Instant::now();
    let duration = (end_time - start_time).as_millis();
    println!("构建花费了 {}", SelfDuration::new(duration));
}
