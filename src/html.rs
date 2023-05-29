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
            return Some(item.inner_html());
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

#[cfg(test)]
mod test {
    use super::*;
    use minify_html::Cfg;

    fn minify(data: &str) -> String {
        let mut cfg = Cfg::new();
        cfg.minify_js = true;
        cfg.minify_css = true;
        cfg.keep_comments = false;

        let res = minify_html::minify(data.as_bytes(), &cfg);
        String::from_utf8(res).unwrap()
    }

    #[test]
    fn test_get_image_url() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_image_urls();
        assert_eq!(res, vec![String::from("assets/UDP_CS模型.png")]);
    }

    #[test]
    fn test_get_title() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_title();
        assert_eq!(res, "套接字");
    }

    #[test]
    fn test_get_toc() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_toc().unwrap();

        assert_eq!(
            minify(&res),
            "<div id=toctitle>Table of Contents</div><ul class=sectlevel1><li><a href=#_注释>注释</a></ul>".to_string()
        );
    }
    #[test]
    fn test_get_footnotes() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_footnotes().unwrap();

        assert_eq!(
            minify(&res),
            "<hr><div class=footnote id=_footnotedef_1><a href=#_footnoteref_1>1</a>. <a href=https://zhuanlan.zhihu.com/p/93609693>深入理解 Epoll</a></div>".to_string()
        );
    }
}
