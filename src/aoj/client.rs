use super::*;

use crate::util;
use crate::Result;

const AOJ_ENDPOINT: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct AojClient;

impl Default for AojClient {
    fn default() -> Self {
        Self
    }
}

impl AojClient {
    pub async fn fetch_problems(&self, page: u32, size: u32) -> Result<Vec<AojProblem>> {
        util::get_json(&format!(
            "{}/problems?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
        .await
    }

    pub async fn fetch_user_ranking(&self, page: u32, size: u32) -> Result<AojUserRanking> {
        util::get_json(&format!(
            "{}/users/ranking/solved?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
        .await
    }

    pub async fn fetch_user_info(&self, user_id: &str) -> Result<AojUserInfo> {
        util::get_json(&format!("{}/users/{}", AOJ_ENDPOINT, user_id)).await
    }

    pub async fn fetch_solutions(
        &self,
        user_id: &str,
        page: u32,
        size: u32,
    ) -> Result<Vec<AojSolution>> {
        util::get_json(&format!(
            "{endpoint}/solutions/users/{user_id}?page={page}&size={size} ",
            endpoint = AOJ_ENDPOINT,
            user_id = user_id,
            page = page,
            size = size
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn problems() {
        let client = AojClient::default();
        let problems = block_on(client.fetch_problems(1, 100)).unwrap();
        assert_eq!(problems.len(), 100);
    }

    #[test]
    fn test_users() {
        let client = AojClient::default();
        let users = block_on(client.fetch_user_ranking(0, 10)).unwrap();
        assert_eq!(users.users.len(), 10);

        let page = (users.total_user_count - 1) / 10;
        let count = (users.total_user_count - 1) % 10;
        let users = block_on(client.fetch_user_ranking(page, 10)).unwrap();
        assert_eq!(users.users.len(), count as usize);
    }

    #[test]
    fn test_find_user() {
        let client = AojClient::default();
        let user = block_on(client.fetch_user_info("kenkoooo"));
        assert!(user.is_ok());

        let user = block_on(client.fetch_user_info("kenkxxxx"));
        assert!(user.is_err());
    }

    #[test]
    fn test_solutions() {
        let client = AojClient::default();
        let solutions = block_on(client.fetch_solutions("kenkoooo", 0, 100)).unwrap();
        assert_eq!(solutions.len(), 100);
    }
}
