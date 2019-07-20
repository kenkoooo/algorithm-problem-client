use crate::util::HtmlClient;
use crate::Result;

use super::*;

use reqwest::Client;

const ATCODER_PREFIX: &str = "https://atcoder.jp";

/// A client for AtCoder.
///
/// # Example
///
/// ```
/// use algorithm_problem_client::atcoder::AtCoderClient;
///
/// let client = AtCoderClient::default();
/// let problems = client.fetch_problem_list("abc107").unwrap();
/// ```
pub struct AtCoderClient {
    client: Client,
}

impl Default for AtCoderClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl AtCoderClient {
    pub fn fetch_contest_list(&self, page: u32) -> Result<Vec<AtCoderContest>> {
        let url = format!("{}/contests/archive?lang=ja&page={}", ATCODER_PREFIX, page);
        let html = self.client.get_html(&url)?;
        contest::scrape(&html)
    }

    /// Fetch a list of submissions.
    pub fn fetch_submission_list(
        &self,
        contest_id: &str,
        page: Option<u32>,
    ) -> Result<AtCoderSubmissionListResponse> {
        let page = page.unwrap_or(1);
        let url = format!(
            "{}/contests/{}/submissions?page={}",
            ATCODER_PREFIX, contest_id, page
        );
        let html = self.client.get_html(&url)?;
        let submissions = submission::scrape(&html, contest_id)?;
        let max_page = submission::scrape_submission_page_count(&html)?;
        Ok(AtCoderSubmissionListResponse {
            max_page,
            submissions,
        })
    }

    pub fn fetch_problem_list(&self, contest_id: &str) -> Result<Vec<AtCoderProblem>> {
        let url = format!("{}/contests/{}/tasks", ATCODER_PREFIX, contest_id);
        let html = self.client.get_html(&url)?;
        problem::scrape(&html, contest_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contest_list() {
        let client = AtCoderClient::default();
        let contests = client.fetch_contest_list(1).unwrap();
        assert_eq!(contests.len(), 50);
    }

    #[test]
    fn test_fetch_problem_list() {
        let client = AtCoderClient::default();
        let problems = client.fetch_problem_list("abc107").unwrap();
        assert_eq!(problems.len(), 4);
    }

    #[test]
    fn test_fetch_submission_list() {
        let client = AtCoderClient::default();
        let response = client.fetch_submission_list("abc134", None).unwrap();
        assert_eq!(response.submissions.len(), 20);

        let response = client
            .fetch_submission_list("abc134", Some(response.max_page))
            .unwrap();
        assert!(!response.submissions.is_empty());

        let response = client.fetch_submission_list("abc134", Some(response.max_page + 1));
        assert!(response.is_err());
    }
}
