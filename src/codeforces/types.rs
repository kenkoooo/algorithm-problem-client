#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeforcesProblem {
    pub id: String,
    pub title: String,
    pub difficulty: Option<u32>,
}
