mod contest;

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

        unimplemented!("{}", html)
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
    ) -> Result<AtCoderContestListResponse> {
        unimplemented!()
    }
}

pub struct AtCoderContestListRequest {
    pub page: u32,
}

pub struct AtCoderContestListResponse {
    pub max_page: u32,
    pub contests: Vec<AtCoderContest>,
}

pub struct AtCoderContest {
    pub id: String,
    pub start_epoch_second: u64,
    pub duration_second: u64,
    pub title: String,
    pub rate_change: String,
}

pub struct AtCoderSubmissionListRequest;
pub struct AtCoderSubmissionListResponse;
pub struct AtCoderProblemListRequest;
pub struct AtCoderProblemListResponse;

pub struct AtCoderSubmission;
pub struct AtCoderProblem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_contest_list() {
        let client = AtCoderClient::new();
        let request = AtCoderContestListRequest { page: 1 };
        let response = client.fetch_contest_list(request).unwrap();
    }
}
