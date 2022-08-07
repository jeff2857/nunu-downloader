use anyhow::{anyhow, Result};

use regex::Regex;

const URL_PREFIX: &str = "https://www.kanunu8.com/";

pub struct UrlParser;

impl UrlParser {
    pub fn validate_index_url(url: &str) -> Result<()> {
        if url.len() <= URL_PREFIX.len() || !url.starts_with(URL_PREFIX) {
            return Err(anyhow!("URL Invalid"));
        }

        let url_path = &url[URL_PREFIX.len()..];
        let re = Regex::new(r"^book\d*/\d+/index.html").unwrap();
        if !re.is_match(url_path) {
            return Err(anyhow!("URL Invalid"));
        }

        Ok(())
    }
}
