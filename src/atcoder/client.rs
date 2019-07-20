use crate::util;
use crate::Result;

use super::*;

const ATCODER_PREFIX: &str = "https://atcoder.jp";

#[derive(Default)]
pub struct AtCoderClient;

impl AtCoderClient {
    pub fn fetch_contest_list(
        &self,
        request: AtCoderContestListRequest,
    ) -> Result<AtCoderContestListResponse> {
        let url = format!(
            "{}/contests/archive?lang=ja&page={}",
            ATCODER_PREFIX, request.page
        );
        let html = util::fetch_html(&url)?;
        let contests = contest::scrape(&html)?;
        Ok(AtCoderContestListResponse { contests })
    }

    pub fn fetch_submission_list(
        &self,
        request: AtCoderSubmissionListRequest,
    ) -> Result<AtCoderSubmissionListResponse> {
        let page = request.page.unwrap_or(1);
        let url = format!(
            "{}/contests/{}/submissions?page={}",
            ATCODER_PREFIX, request.contest_id, page
        );
        let html = util::fetch_html(&url)?;
        let submissions = submission::scrape(&html, request.contest_id)?;
        let max_page = submission::scrape_submission_page_count(&html)?;
        Ok(AtCoderSubmissionListResponse {
            max_page,
            submissions,
        })
    }

    pub fn fetch_problem_list(
        &self,
        request: AtCoderProblemListRequest,
    ) -> Result<AtCoderProblemListResponse> {
        let url = format!("{}/contests/{}/tasks", ATCODER_PREFIX, request.contest_id);
        let html = util::fetch_html(&url)?;
        let problems = problem::scrape(&html, request.contest_id)?;
        Ok(AtCoderProblemListResponse { problems })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contest_list() {
        let client = AtCoderClient::default();
        let request = AtCoderContestListRequest { page: 1 };
        let response = client.fetch_contest_list(request).unwrap();
        assert_eq!(response.contests.len(), 50);
    }

    #[test]
    fn test_fetch_problem_list() {
        let client = AtCoderClient::default();
        let request = AtCoderProblemListRequest {
            contest_id: "abc107",
        };
        let response = client.fetch_problem_list(request).unwrap();
        assert_eq!(response.problems.len(), 4);
    }

    #[test]
    fn test_fetch_submission_list() {
        let client = AtCoderClient::default();

        let request = AtCoderSubmissionListRequest {
            contest_id: "abc134",
            page: None,
        };
        let response = client.fetch_submission_list(request).unwrap();
        assert_eq!(response.submissions.len(), 20);

        let request = AtCoderSubmissionListRequest {
            contest_id: "abc134",
            page: Some(response.max_page),
        };
        let response = client.fetch_submission_list(request).unwrap();
        assert!(!response.submissions.is_empty());

        let request = AtCoderSubmissionListRequest {
            contest_id: "abc134",
            page: Some(response.max_page + 1),
        };
        let response = client.fetch_submission_list(request);
        assert!(response.is_err());
    }
}
