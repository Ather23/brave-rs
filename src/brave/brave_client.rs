use reqwest::RequestBuilder;
use thiserror;
use crate::WebSearchApiResponse;
pub struct BraveClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum BraveClientError {
    #[error("Client error: {0}")] ClientError(String),
    #[error("Http error: {0}")] HttpError(String),
}

impl From<reqwest::Error> for BraveClientError {
    fn from(err: reqwest::Error) -> Self {
        BraveClientError::ClientError(err.to_string())
    }
}

impl BraveClient {
    pub fn new(api_key: &str) -> Self {
        BraveClient {
            api_key: api_key.to_string(),
            base_url: String::from("https://api.search.brave.com/res/v1"),
            client: reqwest::Client::new(),
        }
    }

    pub async fn web_search(&self, query: &str) -> Result<WebSearchApiResponse, BraveClientError> {
        let url = format!("/web/search?q={}", query);
        let response = self.get_request_builder(&url).send().await?;
        let response = match response.error_for_status() {
            Ok(resp) => resp,
            Err(err) => {
                let status = err.status();
                let body = err
                    .url()
                    .map(|_| "Failed to get body due to HTTP error".to_string())
                    .unwrap_or_default();
                return Err(
                    BraveClientError::HttpError(format!("HTTP error {:?}: {}", status, body))
                );
            }
        };

        Ok(response.json::<WebSearchApiResponse>().await?)
    }

    fn get_request_builder(&self, url_path: &str) -> RequestBuilder {
        let mut url = self.base_url.to_owned();
        url.push_str(url_path);
        let response = self.client
            .get(url)
            .header("X-Subscription-Token", &self.api_key)
            .header("Accept", "application/json");
        return response;
    }
}
