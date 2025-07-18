#![allow(dead_code)]

mod config;
use console::Emoji;
use fs_more::{
    directory::{
        copy_directory_with_progress, CollidingSubDirectoryBehaviour, DestinationDirectoryRule,
        DirectoryCopyWithProgressOptions,
    },
    file::CollidingFileBehaviour,
};
use futures::{stream, StreamExt};
use index::index_clear;
pub mod error;
mod filetree;
mod generator;
mod index;
mod utils;
use crate::index::index_list;
use std::path;
use tokio::fs;
use tokio::time::Instant;
use utils::cpu_num;

use clap::Parser;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use lazy_regex::regex;
use tracing::*;

use crate::{
    config::Config, filetree::generate_filetree_json, generator::AdocGenerator, utils::GitInfo,
};
use pagefind::runner::run_indexer;

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
    #[arg(long, default_value = "")]
    theme: String,
    #[arg(short, long, default_value = "./asphinx.toml")]
    config: String,
    #[arg(long, default_value = None)]
    site: Option<String>,
}

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

#[tokio::main]
async fn main() {
    let _ = index_clear();
    let args = Args::parse();
    if args.site.is_some() {
        println!("xxx: {:?}", args.site);
        match run_indexer().await {
            Ok(_) => {}
            Err(msg) => {
                eprintln!("{msg}");
            }
        }
        return;
    }
    assert!(!args.theme.is_empty(), "theme can not be empty");
    init_logger();

    let started = Instant::now();

    let mpb = MultiProgress::new();

    let pb = mpb.add(ProgressBar::new_spinner());

    let gitinfo = GitInfo::new(".".to_string(), pb).await.unwrap();

    let entry_file = "content/index.adoc";

    let config = Config::from_path(args.config).await;
    debug!(?config);
    let generator = AdocGenerator::new(args.theme.clone(), config.clone());

    let pb = mpb.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.set_message("Parse Index info...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let files = parse_index_file(entry_file.into());

    pb.finish_with_message(format!(
        "Parse index completed, total {} file.",
        files.len()
    ));

    let raw_html = {
        let total_files = files.len();
        let pb = mpb.add(ProgressBar::new(total_files as u64));
        pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("━━╾╴─"));
        pb.set_message("Generating HTML ...");

        let raw_html: Vec<_> = stream::iter(files.into_iter().map(|source_file| async {
            let pb_clone = pb.clone();
            {
                pb_clone.set_message(format!("Generated {} ...", source_file));
                let result = generator.generate_html(source_file.into()).await;
                pb_clone.inc(1);
                result
            }
        }))
        .buffer_unordered(cpu_num())
        .filter_map(|v| async { v })
        .collect()
        .await;
        pb.finish_with_message("Generated all files");
        raw_html
    };

    {
        let total_files = raw_html.len();
        let pb = mpb.add(ProgressBar::new(total_files as u64));
        pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("━━╾╴─"));
        pb.set_message("Render HTML ...");

        let _: Vec<_> = stream::iter(raw_html.into_iter().map(|(ctx, html)| async {
            let pb_clone = pb.clone();
            {
                pb_clone.set_message(format!("render {} ...", ctx.source_file));
                let result = generator
                    .render_html(ctx, html, &gitinfo, args.minify)
                    .await;
                pb_clone.inc(1);
                result
            }
        }))
        .buffer_unordered(cpu_num())
        .collect()
        .await;
        pb.finish_with_message("Render all files");
    }

    let asset_path = path::Path::new(&args.theme).join("assets");
    if asset_path.is_dir() {
        let pb = mpb.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.set_message("Copying assets...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let pwd = std::env::current_dir().unwrap();
        let pwd = pwd.join("public/assets");

        let _ = tokio::task::spawn_blocking(move || {
            let ret = copy_directory_with_progress(
                asset_path,
                pwd,
                DirectoryCopyWithProgressOptions {
                    destination_directory_rule: DestinationDirectoryRule::AllowNonEmpty {
                        colliding_file_behaviour: CollidingFileBehaviour::Overwrite,
                        colliding_subdirectory_behaviour: CollidingSubDirectoryBehaviour::Continue,
                    },
                    ..Default::default()
                },
                |process| {
                    pb.set_message(format!(
                        "{}/{}",
                        process.current_operation_index, process.total_operations,
                    ));
                },
            );
            pb.finish_with_message("Copy assets completed.");
            if let Err(e) = ret {
                error!(%e);
            }
        })
        .await;
    }

    let pb = mpb.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.set_message("Generating index file...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    match index_list() {
        Ok(index) => {
            if let Ok(index) = serde_json::to_string(&index) {
                debug!(%index);
                let _ = fs::write("public/cache.json", &index).await;
                pb.finish_with_message("Generated index file.");
            }
        }
        Err(err) => {
            pb.finish_with_message(format!("Generated index file failed: {err}"));
        }
    }

    let pb = mpb.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb.set_message("Generating file tree...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    match generate_filetree_json("content", "public/filetree.json").await {
        Ok(_) => {
            pb.finish_with_message("Generated file tree.");
        }
        Err(err) => {
            pb.finish_with_message(format!("Generated file tree failed: {err}"));
        }
    }
    let cmd = std::env::args_os().next();
    if let Some(cmd) = cmd {
        let mut child = tokio::process::Command::new(cmd)
            .arg("--site")
            .arg("public")
            .spawn()
            .expect("Failed to restart server");
        let _ = child.wait().await;
    }

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
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
