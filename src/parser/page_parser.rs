use regex::Regex;

pub struct PageParser;

impl PageParser {
    pub fn parse_index(body: String) -> Novel {
        let mut novel = Novel::default();
        let title = Self::find_novel_title(&body);
        novel.title = title;

        novel
    }

    pub fn parse_chapter_page(body: String) -> Chapter {
        Chapter::default()
    }

    fn find_novel_title(body: &str) -> String {
        let title_pattern =
            r#"<h1><strong><font color="\#dc143c">(?P<title>.*)</font></strong></h1>"#;
        let re = Regex::new(title_pattern).unwrap();
        let title = &re.captures(body).unwrap()["title"];

        title.to_string()
    }
}

#[derive(Default, Debug)]
pub struct Novel {
    pub title: String,
    pub author: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Default, Debug)]
pub struct Chapter {
    pub title: String,
    pub content: String,
    pub url: String,
}
