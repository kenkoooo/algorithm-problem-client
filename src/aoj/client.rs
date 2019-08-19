use super::*;

use crate::util::HttpClient;
use crate::Result;

const AOJ_ENDPOINT: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct AojClient {
    http_client: HttpClient,
}

impl Default for AojClient {
    fn default() -> Self {
        Self {
            http_client: HttpClient::default(),
        }
    }
}

impl AojClient {
    pub fn fetch_problems(&self, page: u32, size: u32) -> Result<Vec<AojProblem>> {
        self.http_client.get_json(&format!(
            "{}/problems?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    pub fn fetch_user_ranking(&self, page: u32, size: u32) -> Result<AojUserRanking> {
        self.http_client.get_json(&format!(
            "{}/users/ranking/solved?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    pub fn fetch_user_info(&self, user_id: &str) -> Result<AojUserInfo> {
        self.http_client
            .get_json(&format!("{}/users/{}", AOJ_ENDPOINT, user_id))
    }

    pub fn fetch_solutions(&self, user_id: &str, page: u32, size: u32) -> Result<Vec<AojSolution>> {
        self.http_client.get_json(&format!(
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
        let client = AojClient::default();
        let problems = client.fetch_problems(1, 100).unwrap();
        assert_eq!(problems.len(), 100);
    }

    #[test]
    fn test_users() {
        let client = AojClient::default();
        let users = client.fetch_user_ranking(0, 10).unwrap();
        assert_eq!(users.users.len(), 10);

        let page = (users.total_user_count - 1) / 10;
        let count = (users.total_user_count - 1) % 10;
        let users = client.fetch_user_ranking(page, 10).unwrap();
        assert_eq!(users.users.len(), count as usize);
    }

    #[test]
    fn test_find_user() {
        let client = AojClient::default();
        let user = client.fetch_user_info("kenkoooo");
        assert!(user.is_ok());

        let user = client.fetch_user_info("kenkxxxx");
        assert!(user.is_err());
    }

    #[test]
    fn test_solutions() {
        let client = AojClient::default();
        let solutions = client.fetch_solutions("kenkoooo", 0, 100).unwrap();
        assert_eq!(solutions.len(), 100);
    }
}
