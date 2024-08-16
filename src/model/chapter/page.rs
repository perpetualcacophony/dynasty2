#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Page {
    pub name: String,
    pub url: String,
}
