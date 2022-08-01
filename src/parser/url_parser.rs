use std::fmt::Display;

use regex::Regex;

const URL_PREFIX: &str = "https://www.kanunu8.com/";

pub struct UrlParser;

type Result<T> = std::result::Result<T, ParseError>;

pub enum ParseError {
    UrlInvalid,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_str;
        match &self {
            ParseError::UrlInvalid => {
                err_str = "Error: Invalid url".to_string();
            }
        }
        write!(f, "{}", err_str)
    }
}

impl UrlParser {
    pub fn parse_index_page<T: AsRef<str>>(url: T) -> Result<()> {
        let url = url.as_ref();
        Self::validate_index_url(url)?;

        Ok(())
    }

    fn validate_index_url(url: &str) -> Result<()> {
        if url.len() <= URL_PREFIX.len() || !url.starts_with(URL_PREFIX) {
            return Err(ParseError::UrlInvalid);
        }

        let url_path = &url[URL_PREFIX.len()..];
        println!("url_path: {}", url_path);
        let re = Regex::new(r"^book\d*/\d+/index.html").unwrap();
        if !re.is_match(url_path) {
            return Err(ParseError::UrlInvalid);
        }

        Ok(())
    }
}
