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

        novel
    }

    pub fn parse_chapter_page(body: String) -> Chapter {
        Chapter::default()
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
        let chapter_pattern = r#"><a href="(?P<href>.*)">(?P<chp>.*)</a></td>"#;
        let re = Regex::new(chapter_pattern).unwrap();
        for cap in re.captures_iter(body) {
            let chp = &cap["chp"];
            println!("chp: {}", chp);
        }
        vec![]
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
