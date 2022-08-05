use reqwest::StatusCode;

use anyhow::{anyhow, Result};

use crate::parser::page_parser::PageParser;

pub struct Fetcher;

impl Fetcher {
    pub async fn download<T: AsRef<str>>(url: T) -> Result<()> {
        let body = Self::fetch_index_page(url).await?;
        let novel = PageParser::parse_index(body);
        println!("novel: {:#?}", novel);

        for chapter in novel.chapters {
            let url = chapter.url.clone();
            tokio::spawn(async move {
                match Self::fetch_chapter_page(&url).await {
                    Err(err) => {
                        println!("Err: {:?}", err);
                    }
                    Ok(body) => {
                        let chapter_parsed = PageParser::parse_chapter_page(body);
                    }
                }
            });
        }

        Ok(())
    }

    async fn fetch_index_page<T: AsRef<str>>(url: T) -> Result<String> {
        // TODO: change request to `Client`
        // TODO: retry when error happens
        let url = url.as_ref();
        let res = reqwest::get(url).await?;
        let status_code = res.status();
        if status_code != StatusCode::OK {
            return Err(anyhow!("Request Error"));
        }

        let body = res.text_with_charset("gb2312").await?;
        Ok(body)
    }

    async fn fetch_chapter_page<T: AsRef<str>>(url: T) -> Result<String> {
        Ok("".into())
    }
}

#[cfg(test)]
mod tests_fetcher {
    use tokio::runtime::Runtime;

    use super::Fetcher;

    #[test]
    fn test_fetch_index_page() {
        let url = "https://www.kanunu8.com/book3/7055/index.html";
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            tokio::spawn(async move {
                if let Err(err) = Fetcher::fetch_index_page(url).await {
                    println!("Err: {:?}", err);
                }
            });
        })
    }
}
