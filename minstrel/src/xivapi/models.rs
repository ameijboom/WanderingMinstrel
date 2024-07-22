use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    pub page: i16,
    #[serde(rename = "PageNext")]
    pub next: Option<i16>,
    #[serde(rename = "PagePrev")]
    pub previous: Option<i16>,
    #[serde(rename = "PageTotal")]
    pub total: i16,
    pub results: i16,
    pub results_per_page: i16,
    pub results_total: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemResult {
    #[serde(rename = "ID")]
    pub id: i32,
    pub icon: String,
    pub name: String,
    pub url: String,
    pub url_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemSearchResult {
    pub pagination: Pagination,
    pub results: Vec<ItemResult>,
}
