use std::fmt::Display;

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
                err_str = "invalid url".to_string();
            }
        }
        write!(f, "{}", err_str)
    }
}

impl UrlParser {
    pub fn parse_index_page<T: AsRef<str>>(url: T) -> Result<()> {
        let url = url.as_ref();
        if url.len() <= URL_PREFIX.len() || !url.starts_with(URL_PREFIX) {
            return Err(ParseError::UrlInvalid);
        }

        Ok(())
    }
}
