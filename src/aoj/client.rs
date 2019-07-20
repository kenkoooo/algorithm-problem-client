use super::*;

use crate::util::JsonClient;
use crate::Result;

use reqwest::Client;

const AOJ_ENDPOINT: &str = "https://judgeapi.u-aizu.ac.jp";

pub struct AojClient {
    client: Client,
}

impl Default for AojClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl AojClient {
    fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: ::serde::de::DeserializeOwned,
    {
        let result = self.client.get_json(url)?;
        Ok(result)
    }

    pub fn problems(&self, page: u32, size: u32) -> Result<Vec<AojProblem>> {
        self.get_json(&format!(
            "{}/problems?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    pub fn users(&self, page: u32, size: u32) -> Result<AojUserRanking> {
        self.get_json(&format!(
            "{}/users/ranking/solved?page={}&size={}",
            AOJ_ENDPOINT, page, size
        ))
    }

    pub fn find_user(&self, user_id: &str) -> Result<AojUserInfo> {
        self.get_json(&format!("{}/users/{}", AOJ_ENDPOINT, user_id))
    }

    pub fn solutions(&self, user_id: &str, page: u32, size: u32) -> Result<Vec<AojSolution>> {
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
        let client = AojClient::default();
        let problems = client.problems(1, 100).unwrap();
        assert_eq!(problems.len(), 100);
    }

    #[test]
    fn test_users() {
        let client = AojClient::default();
        let users = client.users(0, 10).unwrap();
        assert_eq!(users.users.len(), 10);

        let page = (users.total_user_count - 1) / 10;
        let count = (users.total_user_count - 1) % 10;
        let users = client.users(page, 10).unwrap();
        assert_eq!(users.users.len(), count as usize);
    }

    #[test]
    fn test_find_user() {
        let client = AojClient::default();
        let user = client.find_user("kenkoooo");
        assert!(user.is_ok());

        let user = client.find_user("kenkxxxx");
        assert!(user.is_err());
    }

    #[test]
    fn test_solutions() {
        let client = AojClient::default();
        let solutions = client.solutions("kenkoooo", 0, 100).unwrap();
        assert_eq!(solutions.len(), 100);
    }
}
