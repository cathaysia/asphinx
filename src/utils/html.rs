use std::sync::LazyLock;

use itertools::Itertools;
use scraper::{Html, Selector};

pub struct HtmlParser {
    html: Html,
}

static TAG_CONTENT: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#content").unwrap());
static TAG_BODY: LazyLock<Selector> = LazyLock::new(|| Selector::parse("body").unwrap());
static TAG_IMG: LazyLock<Selector> = LazyLock::new(|| Selector::parse("img").unwrap());
static TAG_FOOTNOTE: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#footnotes").unwrap());
static TAG_TITLE: LazyLock<Selector> = LazyLock::new(|| Selector::parse("title").unwrap());
static TAG_TOC: LazyLock<Selector> = LazyLock::new(|| Selector::parse("#toc").unwrap());

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        Self {
            html: Html::parse_document(html),
        }
    }

    pub fn get_content(&self) -> Option<String> {
        if let Some(v) = self.html.select(&TAG_CONTENT).next() {
            return Some(v.inner_html().trim().into());
        }
        None
    }

    pub fn text(&self) -> String {
        if let Some(v) = self.html.select(&TAG_BODY).next() {
            return v.text().join("");
        }

        Default::default()
    }

    pub fn get_title(&self) -> String {
        if let Some(item) = self.html.select(&TAG_TITLE).next() {
            return item.inner_html().trim().into();
        }
        unreachable!()
    }

    pub fn get_toc(&self) -> Option<String> {
        if let Some(item) = self.html.select(&TAG_TOC).next() {
            return Some(
                item.inner_html()
                    .replace(r#"<div id="toctitle">Table of Contents</div>"#, ""),
            );
        }
        None
    }

    pub fn get_image_urls(&self) -> Vec<String> {
        let mut res = Vec::new();

        for item in self.html.select(&TAG_IMG) {
            if let Some(val) = item.value().attr("src") {
                if let Ok(url) = urlencoding::decode(val) {
                    res.push(url.into());
                }
            }
        }

        res
    }

    pub fn get_footnotes(&self) -> Option<String> {
        if let Some(item) = self.html.select(&TAG_FOOTNOTE).next() {
            return Some(item.inner_html());
        }
        None
    }
}
