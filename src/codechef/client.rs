use crate::util;
use crate::Result;

use super::*;

const CODECHEF_PREFIX: &str = "https://www.codechef.com/problems/";

pub enum CodeChefProblemPage {
    Beginner,
    Easy,
    Medium,
    Hard,
    Challenge,
    Peer,
}

impl CodeChefProblemPage {
    fn value(&self) -> &str {
        match self {
            CodeChefProblemPage::Beginner => "school",
            CodeChefProblemPage::Easy => "easy",
            CodeChefProblemPage::Medium => "medium",
            CodeChefProblemPage::Hard => "hard",
            CodeChefProblemPage::Challenge => "challenge",
            CodeChefProblemPage::Peer => "extcontest",
        }
    }
}

pub struct CodeChefClient;

impl Default for CodeChefClient {
    fn default() -> Self {
        Self
    }
}

impl CodeChefClient {
    pub async fn fetch_problem_list(
        &self,
        page: CodeChefProblemPage,
    ) -> Result<Vec<CodeChefProblem>> {
        let url = format!("{}{}", CODECHEF_PREFIX, page.value());
        let html = util::get_html(&url).await?;
        problem::scrape(&html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_fetch_problem_list() {
        let client = CodeChefClient::default();
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Beginner)).is_ok());
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Easy)).is_ok());
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Medium)).is_ok());
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Hard)).is_ok());
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Challenge)).is_ok());
        assert!(block_on(client.fetch_problem_list(CodeChefProblemPage::Peer)).is_ok());
    }
}
