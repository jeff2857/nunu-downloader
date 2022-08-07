use std::sync::{Arc, Mutex};

use regex::Regex;

pub struct PageParser;

impl PageParser {
    pub fn parse_index(body: String) -> Novel {
        let mut novel = Novel::default();

        let body = Self::trim_body(&body);

        let title = Self::find_novel_title(&body);
        let author = Self::find_novel_author(&body);
        let chapters = Self::find_novel_chapters(&body);

        novel.title = title;
        novel.author = author;
        novel.chapters = chapters;

        novel
    }

    pub fn parse_chapter_page(body: String) -> String {
        let body = Self::trim_chapter_body(&body);
        let mut content = body
            .replace("&nbsp;", " ")
            .replace("&quot;", "\"")
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n");
        //.replace("\"", "");
        content.push_str("\n\n");

        content
    }

    fn trim_body(body: &str) -> String {
        let mut begin = 0;
        for _ in 0..8 {
            let e = body[begin..].find(r"</table>").unwrap();
            begin += e + 1;
        }

        let mut end = begin + body[begin..].rfind(r"努努书坊").unwrap();
        for _ in 0..2 {
            let e = body[begin..end].rfind("</table>").unwrap();
            end = begin + e;
        }

        body[begin..end].to_string()
    }

    fn trim_chapter_body(body: &str) -> String {
        let mut begin = 0;
        for _ in 0..4 {
            let e = body[begin..].find(r"</table>").unwrap();
            begin += e + 1;
        }
        let mut end = begin + body[begin..].find("</table>").unwrap();

        begin += body[begin..end].find("<p>").unwrap() + 3;
        end = begin + body[begin..end].rfind("</p>").unwrap();

        body[begin..end].to_string()
    }

    fn find_novel_title(body: &str) -> String {
        let title_pattern =
            r#"<h1><strong><font color="\#dc143c">(?P<title>.*)</font></strong></h1>"#;
        let re = Regex::new(title_pattern).unwrap();
        let title = &re.captures(body).unwrap()["title"];

        title.to_string()
    }

    fn find_novel_author(body: &str) -> String {
        let author_pattern = r"\u4f5c\u8005\uff1a(?P<author>.*)\s\u53d1\u5e03\u65f6\u95f4";
        let re = Regex::new(author_pattern).unwrap();
        let author = &re.captures(body).unwrap()["author"];

        author.to_string()
    }

    fn find_novel_chapters(body: &str) -> Vec<Chapter> {
        let chapter_pattern = r#"><a href="(?P<href>.*)">(?P<title>.*)</a></td>"#;
        let re = Regex::new(chapter_pattern).unwrap();
        let mut chapters = vec![];

        for cap in re.captures_iter(body) {
            let title = &cap["title"];
            let href = &cap["href"];
            let chapter = Chapter {
                title: title.to_string(),
                url: href.to_string(),
                content: Arc::new(Mutex::new(String::new())),
            };
            chapters.push(chapter);
        }

        chapters
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
    pub content: Arc<Mutex<String>>,
    pub url: String,
}
