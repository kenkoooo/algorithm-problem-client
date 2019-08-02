use super::*;
use crate::error::Result;
use crate::util::HttpClient;

const CODEFORCES_PREFIX: &str = "https://codeforces.com";

pub trait CodeforcesClient {
    fn fetch_problem_list(&self, page: u32) -> Result<(Vec<CodeforcesProblem>, u32)>;
}

impl CodeforcesClient for HttpClient {
    fn fetch_problem_list(&self, page: u32) -> Result<(Vec<CodeforcesProblem>, u32)> {
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
        let (problems, _) = client.fetch_problem_list(1).unwrap();
        assert_eq!(problems.len(), 100);
    }
}
