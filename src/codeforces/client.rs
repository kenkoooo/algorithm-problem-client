use super::*;

use crate::util::HttpClient;
use crate::Result;
use serde::Deserialize;

const BASE_URL: &str = "https://codeforces.com/api";

pub trait CodeforcesClient {
    fn fetch_problems(&self) -> Result<Vec<CodeforcesProblem>>;
    fn fetch_submissions(
        &self,
        user_id: &str,
        from: u32,
        count: u32,
    ) -> Result<Vec<CodeforcesSubmission>>;
}

impl CodeforcesClient for HttpClient {
    fn fetch_problems(&self) -> Result<Vec<CodeforcesProblem>> {
        let url = format!("{}/problemset.problems", BASE_URL);
        self.get_json::<CodeforcesProblemResponse>(&url)
            .map(|response| response.result.problems)
    }

    fn fetch_submissions(
        &self,
        user_id: &str,
        from: u32,
        count: u32,
    ) -> Result<Vec<CodeforcesSubmission>> {
        let url = format!(
            "{base_url}/user.status?handle={user_id}&from={from}&count={count}",
            base_url = BASE_URL,
            user_id = user_id,
            from = from,
            count = count,
        );
        self.get_json::<SubmissionResult>(&url).map(|response| {
            response
                .result
                .into_iter()
                .flat_map(|submission| submission.convert())
                .collect()
        })
    }
}

#[derive(Deserialize)]
struct CodeforcesProblemResponse {
    result: CodeforcesProblemResponseResult,
}

#[derive(Deserialize)]
struct CodeforcesProblemResponseResult {
    problems: Vec<CodeforcesProblem>,
}

#[derive(Deserialize)]
struct SubmissionResult {
    result: Vec<Submission>,
}
#[derive(Deserialize)]
struct Submission {
    id: u64,
    problem: CodeforcesProblem,
    author: Author,

    #[serde(rename = "programmingLanguage")]
    language: String,

    verdict: String,
}
#[derive(Deserialize)]
struct Author {
    members: Vec<Member>,
}
#[derive(Deserialize)]
struct Member {
    handle: String,
}

impl Submission {
    fn convert(self) -> Option<CodeforcesSubmission> {
        let id = self.id;
        let contest_id = self.problem.contest_id;
        let problem_index = self.problem.index;
        let verdict = self.verdict;
        let language = self.language;
        let user_id = self.author.members.into_iter().next()?.handle;
        Some(CodeforcesSubmission {
            id,
            contest_id,
            problem_index,
            user_id,
            verdict,
            language,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_problems() {
        let client = HttpClient::default();
        assert!(client.fetch_problems().unwrap().len() > 0);
    }

    #[test]
    fn test_fetch_submissions() {
        let client = HttpClient::default();
        let submissions = client.fetch_submissions("kenkoooo", 1, 10).unwrap();
        assert_eq!(submissions.len(), 10);
    }
}
