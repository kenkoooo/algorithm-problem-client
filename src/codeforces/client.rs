use super::*;
use crate::error::Result;
use crate::util::HttpClient;

const CODEFORCES_PREFIX: &str = "https://codeforces.com";

pub trait CodeforcesClient {
    fn fetch_problem_list(&self, page: u32) -> Result<CodeforcesProblemList>;
}

impl CodeforcesClient for HttpClient {
    fn fetch_problem_list(&self, page: u32) -> Result<CodeforcesProblemList> {
        let url = format!("{}/problemset/page/{}", CODEFORCES_PREFIX, page);
        let html = self.get_html(&url)?;
        problem::scrape(&html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_problem_list() {
        let client = HttpClient::default();
        let list = client.fetch_problem_list(1).unwrap();
        assert_eq!(list.problems.len(), 100);
    }
}
