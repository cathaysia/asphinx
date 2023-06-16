#![allow(dead_code)]

use env_logger::Env;
use log::*;
use regex::Regex;
use std::path;

mod asciidoctor_builder;
mod duration;
mod generator;
mod git;
mod html;
mod jinjaext;
mod tmpl;
use duration::PrintableDuration;

use clap::Parser;

use crate::{generator::AdocGenerator, git::GitInfo};

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

        let re = Regex::new(r"xref:(.*)\[.*\]").unwrap();
        for item in re.captures_iter(&content) {
            let file_name: String = item.get(1).unwrap().as_str().into();
            let file_path: String = dir_path.join(file_name.as_str()).to_str().unwrap().into();
            result.append(&mut parse_index_file(file_path));
        }
    }

    result.push(file_path_str);

    return result;
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    minify: bool,
    #[arg(long, default_value_t = String::from("layouts"))]
    theme: String,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    let start_time = std::time::SystemTime::now();
    let args = Args::parse();
    assert!(std::path::Path::new(&args.theme).exists());

    let gitinfo = GitInfo::new(".").unwrap();
    let elapsed = start_time.elapsed().unwrap();
    println!("检查 Git 内容花费了 {}", PrintableDuration::from(elapsed));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // TODO: 使用沙盒限制程序能够读取的路径

        let file_path = "content/index.adoc";
        let generator = AdocGenerator::new(args.theme);

        let mut files = parse_index_file(file_path.into());
        let b = files
            .iter_mut()
            .map(|item| generator.generate_html(&gitinfo, item.to_string(), args.minify));

        futures::future::join_all(b).await;
    });

    let elapsed = start_time.elapsed().unwrap();
    println!("构建花费了 {}", PrintableDuration::from(elapsed));
}
