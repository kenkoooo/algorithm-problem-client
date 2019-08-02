use super::*;

use crate::util::HttpClient;
use crate::Result;

const AOJ_ENDPOINT: &str = "https://judgeapi.u-aizu.ac.jp";

pub trait AojClient {
    fn problems(&self, page: u32, size: u32) -> Result<Vec<AojProblem>>;
    fn users(&self, page: u32, size: u32) -> Result<AojUserRanking>;
    fn find_user(&self, user_id: &str) -> Result<AojUserInfo>;
    fn solutions(&self, user_id: &str, page: u32, size: u32) -> Result<Vec<AojSolution>>;
}

impl AojClient for HttpClient {
    fn problems(&self, page: u32, size: u32) -> Result<Vec<AojProblem>> {
        self.get_json(&format!(
            "{}/problems?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    fn users(&self, page: u32, size: u32) -> Result<AojUserRanking> {
        self.get_json(&format!(
            "{}/users/ranking/solved?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    fn find_user(&self, user_id: &str) -> Result<AojUserInfo> {
        self.get_json(&format!("{}/users/{}", AOJ_ENDPOINT, user_id))
    }

    fn solutions(&self, user_id: &str, page: u32, size: u32) -> Result<Vec<AojSolution>> {
        self.get_json(&format!(
            "{endpoint}/solutions/users/{user_id}?page={page}&size={size} ",
            endpoint = AOJ_ENDPOINT,
            user_id = user_id,
            page = page,
            size = size
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problems() {
        let client = HttpClient::default();
        let problems = client.problems(1, 100).unwrap();
        assert_eq!(problems.len(), 100);
    }

    #[test]
    fn test_users() {
        let client = HttpClient::default();
        let users = client.users(0, 10).unwrap();
        assert_eq!(users.users.len(), 10);

        let page = (users.total_user_count - 1) / 10;
        let count = (users.total_user_count - 1) % 10;
        let users = client.users(page, 10).unwrap();
        assert_eq!(users.users.len(), count as usize);
    }

    #[test]
    fn test_find_user() {
        let client = HttpClient::default();
        let user = client.find_user("kenkoooo");
        assert!(user.is_ok());

        let user = client.find_user("kenkxxxx");
        assert!(user.is_err());
    }

    #[test]
    fn test_solutions() {
        let client = HttpClient::default();
        let solutions = client.solutions("kenkoooo", 0, 100).unwrap();
        assert_eq!(solutions.len(), 100);
    }
}
