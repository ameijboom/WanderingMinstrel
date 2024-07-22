use anyhow::anyhow;

use crate::xivapi::models::ItemSearchResult;

pub struct Client {
    req: reqwest::Client,
    base: String,
    key: String,
}

impl Client {
    pub fn new(key: String) -> Self {
        return Client {
            req: reqwest::Client::new(),
            base: "https://xivapi.com".to_string(),
            key,
        };
    }

    async fn get(&self, path: &str) -> anyhow::Result<reqwest::Response> {
        self.req
            .get(format!("{}{}&private_key={}", self.base, path, self.key))
            .send()
            .await
            .map_err(|e| {
                tracing::error!({ path, error = ?e }, "failed to get data");
                anyhow::anyhow!(e)
            })
    }

    pub async fn search_item(&self, name: &str) -> anyhow::Result<ItemSearchResult> {
        self.get(&format!("/search?string={}", name.replace(" ", "+")))
            .await?
            .json::<ItemSearchResult>()
            .await
            .map_err(|e| {
                tracing::error!({ name, error = ?e }, "failed to parse item results");
                anyhow!(e)
            })
    }
}
