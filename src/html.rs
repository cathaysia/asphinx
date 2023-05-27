use minify_html::{minify, Cfg};
use scraper::{Html, Selector};

pub struct HtmlParser {
    html: Html,
    cfg: Cfg,
}

impl HtmlParser {
    pub fn new(html: &str) -> Self {
        let mut cfg = Cfg::new();
        cfg.keep_comments = false;

        Self {
            html: Html::parse_document(html),
            cfg,
        }
    }

    pub fn get_content(self: &Self) -> Option<String> {
        let selector = Selector::parse("#content").unwrap();
        for item in self.html.select(&selector) {
            let res = minify(item.inner_html().as_bytes(), &self.cfg);
            match String::from_utf8(res) {
                Ok(v) => return Some(v),
                Err(_) => return None,
            }
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
            let res = minify(item.inner_html().as_bytes(), &self.cfg);
            match String::from_utf8(res) {
                Ok(v) => return Some(v),
                Err(_) => return None,
            }
        }
        None
    }

    pub fn get_image_urls(self: &Self) -> Vec<String> {
        let selector = Selector::parse("img").unwrap();
        let mut res = Vec::new();
        for item in self.html.select(&selector) {
            res.push(item.value().attr("src").unwrap().into());
        }

        return res;
    }
    pub fn get_footnotes(self: &Self) -> Option<String> {
        let selector = Selector::parse("#footnotes").unwrap();
        for item in self.html.select(&selector) {
            let res = minify(item.inner_html().as_bytes(), &self.cfg);
            match String::from_utf8(res) {
                Ok(v) => return Some(v),
                Err(_) => return None,
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let res = html.get_toc();

        assert_eq!(
            res,
            Some("<div id=toctitle>Table of Contents</div><ul class=sectlevel1><li><a href=#_注释>注释</a></ul>".to_string())
        );
    }
    #[test]
    fn test_get_footnotes() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_footnotes();

        assert_eq!(
            res,
            Some("<hr><div class=footnote id=_footnotedef_1><a href=#_footnoteref_1>1</a>. <a href=https://zhuanlan.zhihu.com/p/93609693>深入理解 Epoll</a></div>".to_string())
        );
    }
}
