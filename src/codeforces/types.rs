use crate::util::Problem;
use serde::Deserialize;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct CodeforcesProblem {
    #[serde(rename = "contestId")]
    pub contest_id: u32,

    pub index: String,
    pub name: String,
    pub rating: Option<u32>,
}

impl Problem for CodeforcesProblem {
    fn url(&self) -> String {
        format!(
            "https://codeforces.com/problemset/problem/{contest_id}/{index}",
            contest_id = self.contest_id,
            index = self.index
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct CodeforcesSubmission {
    pub id: u64,
    pub contest_id: u32,
    pub problem_index: String,
    pub user_id: String,
    pub verdict: String,
    pub language: String,
}
