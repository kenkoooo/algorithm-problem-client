pub struct AtCoderSubmissionListResponse {
    pub max_page: u32,
    pub submissions: Vec<AtCoderSubmission>,
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
pub struct AtCoderSubmission {
    pub id: u64,
    pub epoch_second: u64,
    pub problem_id: String,
    pub contest_id: String,
    pub user_id: String,
    pub language: String,
    pub point: u64,
    pub length: u64,
    pub result: String,
    pub execution_time: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtCoderProblem {
    pub id: String,
    pub title: String,
    pub position: String,
    pub contest_id: String,
}
