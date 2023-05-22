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

    pub fn get_body_of_html(self: &Self) -> Option<String> {
        let selector = Selector::parse("body").unwrap();
        for item in self.html.select(&selector) {
            return Some(item.inner_html().trim().into());
        }
        None
    }

    pub fn get_title_of_html(self: &Self) -> Option<String> {
        let selector = Selector::parse("title").unwrap();
        for item in self.html.select(&selector) {
            return Some(item.inner_html().trim().into());
        }
        None
    }

    pub fn get_image_url(self: &Self) -> Vec<String> {
        let selector = Selector::parse("img").unwrap();
        let mut res = Vec::new();
        for item in self.html.select(&selector) {
            res.push(item.value().attr("src").unwrap().into());
        }

        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_image_url() {
        let html = HtmlParser::new(include_str!("index.html"));
        let res = html.get_image_url();
        assert_eq!(res, vec![String::from("assets/UDP_CS模型.png")]);
    }
}
