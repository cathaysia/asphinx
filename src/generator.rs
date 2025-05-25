use std::path::{self, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::*;

use crate::{
    config,
    index::index_insert,
    utils::{jinjaext, AsciidoctorBuilder, GitInfo, HtmlParser, Tmpl},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Document {
    pub site: String,
    pub title: String,
    pub content: Option<String>,
    pub toc: Option<String>,
    pub footnotes: Option<String>,
    pub last_modify_date: Option<String>,
    pub ancestors: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct BuildContext {
    pub source_dir: String,
    pub source_file: String,
    pub dest_dir: String,
    pub dest_file: String,
}

#[derive(Debug)]
pub struct AdocGenerator {
    engine: Tmpl,
    config: config::Config,
}

impl AdocGenerator {
    pub fn new(theme_dir: String, config: config::Config) -> Self {
        let engine = Tmpl::new(theme_dir);

        Self { engine, config }
    }

    pub async fn render_html(
        &self,
        ctx: BuildContext,
        html: String,
        gitinfo: &GitInfo,
        need_minify: bool,
    ) {
        let BuildContext {
            source_dir,
            source_file,
            dest_dir,
            dest_file,
        } = ctx;
        let html = HtmlParser::new(&html);

        match dest_file.split_once("public/") {
            Some((_, p)) => {
                let _ = index_insert(
                    p.into(),
                    (
                        html.text(),
                        html.get_title(),
                        gitinfo.get_last_commit_time_of_file(&source_file).await,
                    ),
                );
            }
            None => {
                let _ = index_insert(
                    dest_file.clone(),
                    (
                        html.text(),
                        html.get_title(),
                        gitinfo.get_last_commit_time_of_file(&source_file).await,
                    ),
                );
            }
        }

        let title = html.get_title();
        if title == "Untitled" {
            warn!("Title is empty, file: {}", source_file);
        }

        let document = Document {
            site: self.config.site.clone(),
            title,
            content: html.get_content(),
            toc: html.get_toc(),
            footnotes: html.get_footnotes(),
            last_modify_date: gitinfo.get_last_commit_time_of_file(&source_file).await,
            ancestors: Self::generate_paths(&dest_file),
        };

        let document = self.render(&document, need_minify);
        if let Err(err) = fs::write(&dest_file, &document).await {
            eprintln!("Failed write file: {}", err);
        }

        let images = html.get_image_urls();
        let filtered_images: Vec<_> = images
            .iter()
            .filter(|item| !item.starts_with("diag-"))
            .collect();

        if !filtered_images.is_empty() {
            let acts = filtered_images
                .iter()
                .map(|item| Self::move_assets(item, &source_dir, &dest_dir));
            futures::future::join_all(acts).await;
        }
    }

    pub async fn generate_html(&self, source_file: PathBuf) -> Option<(BuildContext, String)> {
        let Ok(ctx) = Self::generate_build_context(source_file) else {
            return None;
        };

        debug!("Generate file: {} -> {}", ctx.source_file, ctx.dest_file);
        let html = Self::generate_raw_page(
            self.config.asciidoc.clone(),
            ctx.source_file.clone(),
            ctx.dest_dir.clone(),
        )
        .await;

        Some((ctx, html))
    }

    pub fn generate_build_context(source_file: PathBuf) -> Result<BuildContext, ()> {
        if !source_file.exists() {
            warn!("Path {} doesn't exists.", source_file.display());
            return Err(());
        }
        if !source_file.is_file() {
            warn!(
                "Path {} is not point to a file, ignore.",
                source_file.display()
            );
            return Err(());
        }

        let source_dir: String = source_file.parent().unwrap().to_str().unwrap().into();
        let dest_dir = source_dir.replace("content", "public");
        if let Err(err) = std::fs::create_dir_all(&dest_dir) {
            error!("Error happens when create file {}: {}", dest_dir, err);
            return Err(());
        }

        let source_file: String = source_file.to_str().unwrap().into();
        let dest_file = source_file
            .replace("content", "public")
            .replace(".adoc", ".html");

        Ok(BuildContext {
            source_dir,
            source_file,
            dest_dir,
            dest_file,
        })
    }

    pub async fn move_assets(item: &str, source: &str, des: &str) {
        let source_file = path::Path::new(source).join(item);
        if !source_file.exists() {
            warn!("File doesn't exists: {}", source_file.display());
            return;
        }
        let des_file = path::Path::new(des).join(item);
        let des_path = des_file.parent().unwrap();
        if !des_path.exists() {
            fs::create_dir_all(des_path).await.unwrap();
        }

        debug!(
            "Move file: {} -> {}",
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

    pub fn generate_paths(dest_file: &str) -> Vec<(String, String)> {
        let paths: Vec<String> = dest_file
            .replace("public/", "")
            .split('/')
            .map(|item| item.to_string())
            .collect();
        let mut res: Vec<(String, String)> = Default::default();
        for idx in 1..paths.len() {
            let a = &paths[0..idx];
            let mut v = a.join("/").to_string();
            v.insert(0, '/');
            res.push((paths[idx - 1].clone(), v));
        }

        res
    }

    pub fn render(&self, context: &Document, need_minify: bool) -> String {
        let tmpl = self.engine.engine.get_template("page").unwrap();
        let ctx = minijinja::value::Value::from_serialize(context);
        let mut res = tmpl.render(ctx).unwrap();
        if need_minify {
            res = jinjaext::minify(&res);
        }

        res
    }
}
