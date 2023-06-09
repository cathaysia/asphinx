use std::path;

use log::*;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    asciidoctor_builder::AsciidoctorBuilder, git::GitInfo, html::HtmlParser, jinjaext, tmpl::Tmpl,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Document {
    title: String,
    content: Option<String>,
    toc: Option<String>,
    footnotes: Option<String>,
    last_modify_date: Option<String>,
    build_date: String,
}

#[derive(Debug)]
pub struct AdocGenerator {
    engine: Tmpl,
}

impl AdocGenerator {
    pub fn new(theme_dir: String) -> Self {
        let engine = Tmpl::new(theme_dir);

        Self { engine }
    }

    pub async fn generate_html(&self, gitinfo: &GitInfo, file_path: String, need_minify: bool) {
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
        let output = AsciidoctorBuilder::new(file_path.clone(), des_dir.clone())
            .enable_toc()
            .enable_diagram()
            .attr("icons=font".into())
            .build()
            .await;

        let html = HtmlParser::new(&output);

        let now = chrono::Utc::now();

        let data = Document {
            title: html.get_title(),
            content: html.get_content(),
            toc: html.get_toc(),
            footnotes: html.get_footnotes(),
            // last_modify_date: git::get_last_commit_time(&file_path).await,
            last_modify_date: gitinfo.get_last_commit_time_of_file(&file_path),
            build_date: format!("{}", now.format("%Y-%m-%d %H:%M:%S")),
        };

        let tmpl = self.engine.engine.get_template("single").unwrap();
        let ctx = minijinja::value::Value::from_serializable(&data);
        let mut res = tmpl.render(ctx).unwrap();
        if need_minify {
            res = jinjaext::minify_inner(&res).unwrap().to_string();
        }

        if let Err(err) = fs::write(&file_des_path, &res).await {
            eprintln!("写入文件失败：{}", err);
        }

        let assets = html.get_image_urls();
        let acts = assets
            .iter()
            .filter(|item| !item.starts_with("diag-"))
            .map(|item| Self::move_assets(&item, &file_cwd, &des_dir));
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
}
