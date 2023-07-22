#[cfg(test)]
mod test {
    use minify_html::Cfg;

    use crate::utils::HtmlParser;

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
            "<ul class=sectlevel1><li><a href=#_注释>注释</a></ul>".to_string()
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
