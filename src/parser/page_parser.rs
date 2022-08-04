pub struct PageParser;

impl PageParser {
    pub fn parse_index(body: String) -> Novel {
        Novel::default()
    }

    pub fn parse_chapter_page(body: String) -> Chapter {
        Chapter::default()
    }
}

#[derive(Default)]
pub struct Novel {
    pub title: String,
    pub author: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Default)]
pub struct Chapter {
    pub title: String,
    pub content: String,
    pub url: String,
}
