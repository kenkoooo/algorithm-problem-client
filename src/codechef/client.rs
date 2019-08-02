use crate::util::HtmlClient;
use crate::Result;
use reqwest::Client;

const CODECHEF_PREFIX: &str = "https://www.codechef.com/problems/";

pub struct CodeChefClient {
    client: Client,
}

impl Default for CodeChefClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl CodeChefClient {
    pub fn fetch_problem_list(&self) -> Result<()> {
        let url = format!("{}{}", CODECHEF_PREFIX, "school");
        let html = self.client.get_html(&url)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_problem_list() {}
}
