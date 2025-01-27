#![allow(dead_code)]

mod config;
use futures::future;
use index::{index_clear, index_list};
use itertools::Itertools;
pub mod error;
mod generator;
mod index;
mod utils;
use tokio::fs;

use std::path;

use clap::Parser;
use lazy_regex::regex;
use tracing::*;

use crate::{
    config::Config,
    generator::AdocGenerator,
    utils::{Counter, GitInfo},
};

fn parse_index_file(file_path_str: String) -> Vec<String> {
    let mut result = Vec::<String>::new();
    debug!("process file: {}", file_path_str);

    let file_path = path::Path::new(&file_path_str);
    if !file_path.exists() {
        warn!("file doesn't existes: {}", file_path.display());
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
    #[arg(long)]
    theme: String,
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
    let _ = index_clear();
    let args = Args::parse();
    init_logger();

    let mut timer = Counter::new();
    let gitinfo = GitInfo::new(".").unwrap();
    println!("checking Git took {}.", timer.elapsed().unwrap());

    let entry_file = "content/index.adoc";

    let config = Config::from_path("asphinx.toml").await;
    debug!(?config);
    let generator = AdocGenerator::new(args.theme.clone(), config.asciidoc);

    timer.reset();
    let files = parse_index_file(entry_file.into());
    println!("parsing index took {}.", timer.elapsed().unwrap());

    let tasks = files
        .into_iter()
        .map(|source_file| generator.generate_html(&gitinfo, source_file.into(), args.minify))
        .collect_vec();

    let cpu_num = std::thread::available_parallelism()
        .map(|item| item.get())
        .unwrap_or(16);
    for i in chunked(tasks, cpu_num) {
        future::join_all(i.into_iter()).await;
    }

    match index_list() {
        Ok(index) => {
            if let Ok(index) = serde_json::to_string(&index) {
                debug!(%index);
                let _ = fs::write("public/cache.json", &index).await;
            }
        }
        Err(err) => {
            error!(%err);
        }
    }

    println!("build took {}.", timer.since_start().unwrap());
}

fn init_logger() {
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
    };
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or("INFO".into())
        .to_lowercase();

    let env_filter = EnvFilter::builder()
        .parse(format!("RUST_LOG=OFF,asphinx={}", log_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer().with_file(true).with_line_number(true))
        .with(env_filter)
        .init();
}
