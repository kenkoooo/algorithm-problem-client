use crate::Result;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING};
use reqwest::Client;
use serde::de::DeserializeOwned;

pub struct HttpClient {
    client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl HttpClient {
    pub fn get_html(&self, url: &str) -> Result<String> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("text/html"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        let mut response = self.client.get(url).headers(headers).send()?;
        let body = response.text()?;
        Ok(body)
    }

    pub fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
        let mut response = self.client.get(url).headers(headers).send()?;
        let body = response.json()?;
        Ok(body)
    }
}
