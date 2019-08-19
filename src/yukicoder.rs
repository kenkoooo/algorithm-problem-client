use crate::util::{HttpClient, Problem};
use crate::Result;
use serde::Deserialize;

const BASE_URL: &str = "https://yukicoder.me/api/v1";

pub trait YukicoderClient {
    fn fetch_problems(&self) -> Result<Vec<YukicoderProblem>>;
}

impl YukicoderClient for HttpClient {
    fn fetch_problems(&self) -> Result<Vec<YukicoderProblem>> {
        let url = format!("{}/problems", BASE_URL);
        self.get_json(&url)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct YukicoderProblem {
    #[serde(rename = "No")]
    number: u32,
    #[serde(rename = "ProblemId")]
    id: u32,
    #[serde(rename = "Title")]
    title: String,
}

impl Problem for YukicoderProblem {
    fn url(&self) -> String {
        format!("https://yukicoder.me/problems/no/{}", self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_problems() {
        let client = HttpClient::default();
        assert!(client.fetch_problems().is_ok());
    }
}
