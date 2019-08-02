use crate::util::Problem;

pub struct CodeforcesProblemList {
    pub problems: Vec<CodeforcesProblem>,
    pub max_page_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeforcesProblem {
    pub id: String,
    pub title: String,
    pub difficulty: Option<u32>,
}

impl Problem for CodeforcesProblem {
    fn url(&self) -> String {
        let prefix: String = self.id.chars().take_while(|c| c.is_numeric()).collect();
        let suffix: String = self.id.chars().skip_while(|c| c.is_numeric()).collect();
        format!(
            "https://codeforces.com/problemset/problem/{}/{}",
            prefix, suffix
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let problem = CodeforcesProblem {
            id: "123B2".to_string(),
            title: "".to_string(),
            difficulty: None,
        };
        assert_eq!(
            "https://codeforces.com/problemset/problem/123/B2".to_string(),
            problem.url()
        );
    }
}
