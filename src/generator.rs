use std::path::{self, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::*;

use crate::{
    config,
    utils::{jinjaext, AsciidoctorBuilder, GitInfo, HtmlParser, Tmpl},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Document {
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
    config: config::Asciidoc,
}

impl AdocGenerator {
    pub fn new(theme_dir: String, config: config::Asciidoc) -> Self {
        let engine = Tmpl::new(theme_dir);

        Self { engine, config }
    }

    pub async fn generate_html(&self, gitinfo: &GitInfo, source_file: PathBuf, need_minify: bool) {
        let Ok(BuildContext {
            source_dir,
            source_file,
            dest_dir,
            dest_file,
        }) = Self::generate_build_context(source_file)
        else {
            return;
        };

        debug!("Generate file: {} -> {}", source_file, dest_file);
        let html =
            Self::generate_raw_page(self.config.clone(), source_file.clone(), dest_dir.clone())
                .await;

        let html = HtmlParser::new(&html);

        let document = Document {
            title: html.get_title(),
            content: html.get_content(),
            toc: html.get_toc(),
            footnotes: html.get_footnotes(),
            last_modify_date: gitinfo.get_last_commit_time_of_file(&source_file),
            ancestors: Self::generate_pathes(&dest_file),
        };

        let document = self.render(&document, need_minify);
        if let Err(err) = fs::write(&dest_file, &document).await {
            eprintln!("Failed write file: {}", err);
        }

        let images = html.get_image_urls();
        let acts = images
            .iter()
            .filter(|item| !item.starts_with("diag-"))
            .map(|item| Self::move_assets(item, &source_dir, &dest_dir));
        futures::future::join_all(acts).await;
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
            error!("Error happend whe create file {}: {}", dest_dir, err);
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
        let tmpl = self.engine.engine.get_template("page").unwrap();
        let ctx = minijinja::value::Value::from_serialize(context);
        let mut res = tmpl.render(ctx).unwrap();
        if need_minify {
            res = jinjaext::minify(&res);
        }

        res
    }
}
