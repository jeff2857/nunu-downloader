use std::env;
use std::sync::Arc;

use reqwest::StatusCode;

use anyhow::{anyhow, Result};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::parser::page_parser::{Novel, PageParser};

static mut INDEX_URL: &str = "";

pub struct Fetcher;

impl Fetcher {
    pub async fn download<T: AsRef<str>>(url: T, file_path: Option<&str>) -> Result<()> {
        let novel = match Self::fetch(url).await {
            Err(err) => {
                return Err(err);
            }
            Ok(novel) => novel,
        };

        match Self::save_to_file(&novel, file_path).await {
            Err(err) => {
                return Err(err);
            }
            Ok(_) => {
                println!("Downloading {} finished", novel.title);
            }
        }

        Ok(())
    }

    pub async fn fetch<T: AsRef<str>>(url: T) -> Result<Novel> {
        let body = Self::fetch_index_page(url).await?;
        let novel = PageParser::parse_index(body);

        println!("Downloading 《{}》 作者: {} ...", novel.title, novel.author);

        for chapter in &novel.chapters {
            let url = chapter.url.clone();
            let chapter_title = chapter.title.clone();
            let chapter_content = Arc::clone(&chapter.content);
            tokio::spawn(async move {
                match Self::fetch_chapter_page(&url, Some(&chapter_title)).await {
                    Err(err) => {
                        println!("Err: {:?}", err);
                    }
                    Ok(body) => {
                        let chapter_parsed = PageParser::parse_chapter_page(body);
                        let mut content = chapter_content.lock().unwrap();
                        *content = chapter_parsed;
                    }
                }
            })
            .await
            .unwrap();
        }

        Ok(novel)
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

        unsafe {
            INDEX_URL = Box::leak(url.replace("index.html", "").into_boxed_str());
        }

        let body = res.text_with_charset("gb2312").await?;
        Ok(body)
    }

    async fn fetch_chapter_page<T: AsRef<str>>(url: T, title: Option<&str>) -> Result<String> {
        let url = unsafe { format!("{}{}", INDEX_URL, url.as_ref()) };

        if let Some(title) = title {
            println!("Downloading {} ...", title);
        }

        let res = reqwest::get(url).await?;
        let status_code = res.status();
        if status_code != StatusCode::OK {
            return Err(anyhow!("Request Error"));
        }

        let body = res.text_with_charset("gb2312").await?;
        Ok(body)
    }

    pub async fn save_to_file(novel: &Novel, path: Option<&str>) -> Result<()> {
        let path = match path {
            Some(p) => {
                let mut p = p.to_string();
                if !p.ends_with(".txt") {
                    p.push_str(".txt");
                }
                p
            }
            None => {
                let mut current_dir = env::current_dir().unwrap();
                current_dir = current_dir.join(format!("{}_{}.txt", novel.title, novel.author));
                current_dir.to_str().unwrap().to_owned()
            }
        };
        let mut file = File::create(path).await?;

        for chp in &novel.chapters {
            let content = Arc::clone(&chp.content);
            let content = content.lock().unwrap();
            let content = content.clone();
            file.write_all(content.as_bytes()).await?;
        }

        Ok(())
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
