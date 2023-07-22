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

    pub fn get_content(self: &Self) -> Option<String> {
        let selector = Selector::parse("#content").unwrap();
        for item in self.html.select(&selector) {
            return Some(item.inner_html());
        }
        None
    }

    pub fn get_title(self: &Self) -> String {
        let selector = Selector::parse("title").unwrap();
        for item in self.html.select(&selector) {
            return item.inner_html().trim().into();
        }
        unreachable!()
    }

    pub fn get_toc(self: &Self) -> Option<String> {
        let selector = Selector::parse("#toc").unwrap();
        for item in self.html.select(&selector) {
            return Some(
                item.inner_html()
                    .replace(r#"<div id="toctitle">Table of Contents</div>"#, ""),
            );
        }
        None
    }

    pub fn get_image_urls(self: &Self) -> Vec<String> {
        let selector = Selector::parse("img").unwrap();
        let mut res = Vec::new();
        for item in self.html.select(&selector) {
            let val = item
                .value()
                .attr("src")
                .unwrap()
                .to_string()
                .replace("%20", " ");
            res.push(val);
        }

        return res;
    }

    pub fn get_footnotes(self: &Self) -> Option<String> {
        let selector = Selector::parse("#footnotes").unwrap();
        for item in self.html.select(&selector) {
            return Some(item.inner_html());
        }
        None
    }
}
