mod contest;
mod problem;

use crate::util;
use crate::Result;

const ATCODER_PREFIX: &str = "https://atcoder.jp";

pub struct AtCoderClient;

impl AtCoderClient {
    pub fn new() -> Self {
        Self
    }

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
        unimplemented!()
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

pub struct AtCoderContestListRequest {
    pub page: u32,
}

pub struct AtCoderContestListResponse {
    pub contests: Vec<AtCoderContest>,
}

pub struct AtCoderSubmissionListRequest;

pub struct AtCoderSubmissionListResponse;
pub struct AtCoderProblemListRequest<'a> {
    pub contest_id: &'a str,
}

pub struct AtCoderProblemListResponse {
    pub problems: Vec<AtCoderProblem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtCoderContest {
    pub id: String,
    pub start_epoch_second: u64,
    pub duration_second: u64,
    pub title: String,
    pub rate_change: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtCoderSubmission;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtCoderProblem {
    pub id: String,
    pub title: String,
    pub position: String,
    pub contest_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contest_list() {
        let client = AtCoderClient::new();
        let request = AtCoderContestListRequest { page: 1 };
        let response = client.fetch_contest_list(request).unwrap();
        assert_eq!(response.contests.len(), 50);
    }

    #[test]
    fn test_fetch_problem_list() {
        let client = AtCoderClient::new();
        let request = AtCoderProblemListRequest {
            contest_id: "abc107",
        };
        let response = client.fetch_problem_list(request).unwrap();
        assert_eq!(response.problems.len(), 4);
    }
}
