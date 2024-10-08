#![allow(dead_code)]

mod config;
use futures::future;
use itertools::Itertools;
pub mod error;
mod generator;
mod utils;

use std::path;

use clap::Parser;
use lazy_regex::regex;
use log::*;
use tokio::fs;
use tracing_subscriber::{
    fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use crate::{
    config::Config,
    generator::AdocGenerator,
    utils::{Counter, GitInfo},
};

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
    theme: Option<String>,
}

// https://users.rust-lang.org/t/how-to-breakup-an-iterator-into-chunks/87915
fn chunked<I>(a: impl IntoIterator<Item = I>, chunk_size: usize) -> impl Iterator<Item = Vec<I>> {
    let mut a = a.into_iter();
    std::iter::from_fn(move || {
        Some(a.by_ref().take(chunk_size).collect()).filter(|chunk: &Vec<_>| !chunk.is_empty())
    })
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    init_logger();

    let mut counter = Counter::new();
    let gitinfo = GitInfo::new(".").unwrap();
    println!("Checking Git took {}.", counter.elapsed().unwrap());

    let file_path = "content/index.adoc";

    let config: config::Config = Config::from_file("config.toml").await;
    let generator = AdocGenerator::new(args.theme, config.asciidoc);

    counter.reset();
    let files = parse_index_file(file_path.into());
    println!("Parsing index took {}.", counter.elapsed().unwrap());
    let b = files
        .into_iter()
        .map(|item| generator.generate_html(&gitinfo, item.into(), args.minify))
        .collect_vec();

    let cpu_num = std::thread::available_parallelism()
        .map(|item| item.get())
        .unwrap_or(16);
    for i in chunked(b, cpu_num) {
        future::join_all(i.into_iter()).await;
    }

    let _ = fs::create_dir_all("public/assets/").await;
    let _ = fs::write(
        "public/assets/breadcrumb.css",
        utils::jinjaext::minify_inner(include_str!("../builtin/assets/breadcrumb.css"))
            .unwrap()
            .to_string(),
    )
    .await;
    let _ = fs::write(
        "public/assets/index.css",
        utils::jinjaext::minify_inner(include_str!("../builtin/assets/index.css"))
            .unwrap()
            .to_string(),
    )
    .await;
    let _ = fs::write(
        "public/assets/prism.css",
        utils::jinjaext::minify_inner(include_str!("../builtin/assets/prism.css"))
            .unwrap()
            .to_string(),
    )
    .await;

    println!("Build took {}.", counter.since_start().unwrap());
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("INFO".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,asphinx={}", log_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();
}
