use serde::Deserialize;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojProblem {
    id: String,
    name: String,

    #[serde(rename = "solvedUser")]
    solved_user: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojUser {
    pub id: String,
    pub name: String,
    pub solved: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojUserRanking {
    #[serde(rename = "outOf")]
    pub total_user_count: u32,
    pub users: Vec<AojUser>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojUserInfo {
    pub id: String,
    pub name: String,
    pub status: AojUserStatus,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojUserStatus {
    pub solved: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Deserialize)]
pub struct AojSolution {
    #[serde(rename = "judgeId")]
    judge_id: u64,

    #[serde(rename = "userId")]
    user_id: String,

    #[serde(rename = "problemId")]
    problem_id: String,

    language: String,
    version: String,

    #[serde(rename = "submissionDate")]
    submission_date: u64,

    #[serde(rename = "judgeDate")]
    judge_date: u64,
}
