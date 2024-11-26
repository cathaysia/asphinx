use scraper::{Html, Selector};

pub struct HtmlParser {
    html: Html,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        Self {
            html: Html::parse_document(html),
        }
    }

    pub fn get_content(&self) -> Option<String> {
        let selector = Selector::parse("#content").unwrap();
        if let Some(v) = self.html.select(&selector).next() {
            return Some(v.inner_html().trim().into());
        }
        None
    }

    pub fn get_title(&self) -> String {
        let selector = Selector::parse("title").unwrap();
        if let Some(item) = self.html.select(&selector).next() {
            return item.inner_html().trim().into();
        }
        unreachable!()
    }

    pub fn get_toc(&self) -> Option<String> {
        let selector = Selector::parse("#toc").unwrap();
        if let Some(item) = self.html.select(&selector).next() {
            return Some(
                item.inner_html()
                    .replace(r#"<div id="toctitle">Table of Contents</div>"#, ""),
            );
        }
        None
    }

    pub fn get_image_urls(&self) -> Vec<String> {
        let selector = Selector::parse("img").unwrap();
        let mut res = Vec::new();

        for item in self.html.select(&selector) {
            if let Some(val) = item.value().attr("src") {
                if let Ok(url) = urlencoding::decode(val) {
                    res.push(url.into());
                }
            }
        }

        res
    }

    pub fn get_footnotes(&self) -> Option<String> {
        let selector = Selector::parse("#footnotes").unwrap();
        if let Some(item) = self.html.select(&selector).next() {
            return Some(item.inner_html());
        }
        None
    }
}
