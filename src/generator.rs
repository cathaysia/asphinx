use std::path::{self, PathBuf};

use chrono::FixedOffset;
use log::*;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    config,
    utils::{jinjaext, AsciidoctorBuilder, GitInfo, HtmlParser, Tmpl},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub content: Option<String>,
    pub toc: Option<String>,
    pub footnotes: Option<String>,
    pub last_modify_date: Option<String>,
    pub build_date: String,
    pub ancestors: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct AdocGenerator {
    engine: Tmpl,
    config: config::Asciidoc,
}

impl AdocGenerator {
    pub fn new(theme_dir: String, config: config::Asciidoc) -> Self {
        let engine = Tmpl::new(theme_dir);

        Self { engine, config }
    }

    pub async fn generate_html(&self, gitinfo: &GitInfo, source_file: PathBuf, need_minify: bool) {
        if !source_file.exists() {
            warn!("路径 {} 不存在", source_file.display());
            return;
        }
        if !source_file.is_file() {
            warn!("路径 {} 指向的不是一个文件，忽略", source_file.display());
            return;
        }

        let source_dir: String = source_file.parent().unwrap().to_str().unwrap().into();
        let des_dir = source_dir.replace("content", "public");
        if let Err(err) = std::fs::create_dir_all(&des_dir) {
            error!("创建 {} 时发生错误：{}", des_dir, err);
            return;
        }

        let source_file: String = source_file.to_str().unwrap().into();
        let dest_file = source_file
            .replace("content", "public")
            .replace(".adoc", ".html");

        info!("生成文件：{} -> {}", source_file, dest_file);
        let output =
            Self::generate_raw_page(self.config.clone(), source_file.clone(), des_dir.clone())
                .await;

        let html = HtmlParser::new(&output);
        let now = chrono::Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());

        let data = Document {
            title: html.get_title(),
            content: html.get_content(),
            toc: html.get_toc(),
            footnotes: html.get_footnotes(),
            last_modify_date: gitinfo.get_last_commit_time_of_file(&source_file),
            build_date: format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
            ancestors: Self::generate_pathes(&dest_file),
        };

        let res = self.render(&data, need_minify);
        if let Err(err) = fs::write(&dest_file, &res).await {
            eprintln!("写入文件失败：{}", err);
        }

        let assets = html.get_image_urls();
        let acts = assets
            .iter()
            .filter(|item| !item.starts_with("diag-"))
            .map(|item| Self::move_assets(item, &source_dir, &des_dir));
        futures::future::join_all(acts).await;
    }

    pub async fn move_assets(item: &str, source: &str, des: &str) {
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

    pub async fn generate_raw_page(
        config: config::Asciidoc,
        source_file: String,
        des: String,
    ) -> String {
        let mut output = AsciidoctorBuilder::new(source_file, des);
        config.attributes.iter().for_each(|(key, value)| {
            match value {
                toml::Value::String(value) => output.attr(format!("{}={}", key, value)),
                _ => output.attr(format!("{}={}", key, value)),
            };
        });
        config.extensions.iter().for_each(|value| {
            output.plugin(value.clone());
        });
        output.build().await
    }

    pub fn generate_pathes(dest_file: &str) -> Vec<(String, String)> {
        let pathes: Vec<String> = dest_file
            .replace("public/", "")
            .split('/')
            .map(|item| item.to_string())
            .collect();
        let mut res: Vec<(String, String)> = Default::default();
        for idx in 1..pathes.len() {
            let a = &pathes[0..idx];
            let mut v = a.join("/").to_string();
            v.insert(0, '/');
            res.push((pathes[idx - 1].clone(), v));
        }

        res
    }

    pub fn render(&self, context: &Document, need_minify: bool) -> String {
        let tmpl = self.engine.engine.get_template("single").unwrap();
        let ctx = minijinja::value::Value::from_serializable(&context);
        let mut res = tmpl.render(ctx).unwrap();
        if need_minify {
            res = jinjaext::minify_inner(&res).unwrap().to_string();
        }

        res
    }
}