use crate::Result;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING};
use reqwest::Client;
use serde::de::DeserializeOwned;

pub(crate) trait HtmlClient {
    fn get_html(&self, url: &str) -> Result<String>;
}

impl HtmlClient for Client {
    fn get_html(&self, url: &str) -> Result<String> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("text/html"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        let mut response = self.get(url).headers(headers).send()?;
        let body = response.text()?;
        Ok(body)
    }
}

pub(crate) trait JsonClient {
    fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned;
}

impl JsonClient for Client {
    fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        let mut response = self.get(url).headers(headers).send()?;
        let body = response.json()?;
        Ok(body)
    }
}
