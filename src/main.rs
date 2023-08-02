#![allow(dead_code)]

mod config;
mod generator;
mod utils;

use std::path;

use clap::Parser;
use lazy_regex::regex;
use log::*;
use mimalloc::MiMalloc;

use crate::{
    generator::AdocGenerator,
    utils::{Counter, GitInfo},
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn parse_index_file(file_path_str: String) -> Vec<String> {
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

        let re = regex!(r"xref:(.*)\[.*\]");
        for item in re.captures_iter(&content) {
            let file_name: String = item.get(1).unwrap().as_str().replace("{cpp}", "c++");
            let file_path: String = dir_path.join(file_name.as_str()).to_str().unwrap().into();
            result.append(&mut parse_index_file(file_path));
        }
    }

    result.push(file_path_str);

    result
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    minify: bool,
    #[arg(long, default_value_t = String::from("layouts"))]
    theme: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let args = Args::parse();
    assert!(std::path::Path::new(&args.theme).exists());

    let mut counter = Counter::new();
    let gitinfo = GitInfo::new(".").unwrap();
    println!("检查 Git 内容花费了 {}", counter.elapsed().unwrap());

    let file_path = "content/index.adoc";

    let config: config::Config =
        toml::from_str(std::fs::read_to_string("config.toml").unwrap().as_str()).unwrap();
    let generator = AdocGenerator::new(args.theme, config.asciidoc);

    counter.reset();
    let files = parse_index_file(file_path.into());
    println!("解析 index 文件花费了 {}", counter.elapsed().unwrap());
    let b = files
        .into_iter()
        .map(|item| generator.generate_html(&gitinfo, item, args.minify));

    futures::future::join_all(b).await;

    println!("构建花费了 {}", counter.since_start().unwrap());
}
