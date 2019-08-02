use crate::util::Problem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeChefProblem {
    pub title: String,
    pub code: String,
    pub successful_counts: u32,
}

impl Problem for CodeChefProblem {
    fn url(&self) -> String {
        format!("https://www.codechef.com/problems/{}", self.code)
    }
}
