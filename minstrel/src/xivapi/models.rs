use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    #[serde(rename = "Page")]
    pub page: i16,
    #[serde(rename = "PageNext")]
    pub next: Option<i16>,
    #[serde(rename = "PagePrev")]
    pub previous: Option<i16>,
    #[serde(rename = "PageTotal")]
    pub total: i16,
    #[serde(rename = "Results")]
    pub results: i16,
    #[serde(rename = "ResultsPerPage")]
    pub results_per_page: i16,
    #[serde(rename = "ResultsTotal")]
    pub results_total: i64
}
#[derive(Serialize, Deserialize)]
pub struct ItemResult {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Icon")]
    pub icon: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "UrlType")]
    pub url_type: String
}

#[derive(Serialize, Deserialize)]
pub struct ItemSearchResult {
    #[serde(rename = "Pagination")]
    pub pagination: Pagination,
    #[serde(rename = "Results")]
    pub results: Vec<ItemResult>
}