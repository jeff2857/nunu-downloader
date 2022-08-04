use reqwest::StatusCode;

use anyhow::{anyhow, Result};

pub struct Fetcher;

impl Fetcher {
    pub async fn fetch_index_page<T: AsRef<str>>(url: T) -> Result<()> {
        // TODO: change request to `Client`
        let url = url.as_ref();
        let res = reqwest::get(url).await?;
        let status_code = res.status();
        if status_code != StatusCode::OK {
            return Err(anyhow!("Request Error"));
        }

        let body = res.text_with_charset("gb2312").await?;
        println!("body: {}", body);

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
