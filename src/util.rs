use crate::{Error, Result};

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING};
use reqwest::Client;

pub(crate) fn fetch_html(url: &str) -> Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("text/html"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));
    let mut response = Client::new().get(url).headers(headers).send()?;
    let body = response.text()?;
    Ok(body)
}
