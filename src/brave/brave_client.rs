use reqwest::RequestBuilder;

use crate::{
    brave::{ query_builders::web_search_query_builder, BraveClientError },
    types::{ query_params::WebSearchQueryParamsBuilder, WebSearchQueryParams },
    WebSearchApiResponse,
};
pub struct BraveClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl BraveClient {
    pub fn new(api_key: &str) -> Self {
        BraveClient {
            api_key: api_key.to_string(),
            base_url: String::from("https://api.search.brave.com/res/v1"),
            client: reqwest::Client::new(),
        }
    }

    pub async fn web_search(
        &self,
        query_params: &WebSearchQueryParams
    ) -> Result<WebSearchApiResponse, BraveClientError> {
        let query = web_search_query_builder(query_params).unwrap();
        let response = self.get_request_builder(&query).send().await?.error_for_status()?;

        let result = response.json::<WebSearchApiResponse>().await?;
        Ok(result)
    }

    pub async fn web_search_by_query(
        &self,
        query: &str
    ) -> Result<WebSearchApiResponse, BraveClientError> {
        let params = WebSearchQueryParamsBuilder::default().q(query).build().unwrap();
        let query: String = params.into();
        let response = self.get_request_builder(&query).send().await?.error_for_status()?;

        let result = response.json::<WebSearchApiResponse>().await?;
        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use httpmock::Method::GET;
    use serde_json::json;

    #[tokio::test]
    async fn test_web_search_by_query() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/web/search").query_param("q", "rust");
            then.status(200)
                .header("Content-Type", "application/json")
                .json_body(
                    json!({
                    "type": "web",
                    "query": { "original": "rust", "show_strict_warning": false, "is_navigational": false, "is_news_breaking": false, "spellcheck_off": false, "country": "us", "bad_results": false, "should_fallback": false, "postal_code": "", "city": "", "header_country": "", "more_results_available": false, "state": "" },
                    "web": { "type": "web", "results": [] }
                })
                );
        });

        let mut client = BraveClient::new("test_key");
        client.base_url = server.base_url();

        let result = client.web_search_by_query("rust").await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.result_type, "web");
        mock.assert();
    }

    #[tokio::test]
    async fn test_web_search_returns_brave_client_error_on_http_error() {
        let server = MockServer::start();

        // Mock a 401 Unauthorized error from the API
        let mock = server.mock(|when, then| {
            when.method(GET).path("/web/search").query_param("q", "unauthorized");
            then.status(401)
                .header("Content-Type", "application/json")
                .json_body(json!({
                "error": "Unauthorized"
            }));
        });

        let mut client = BraveClient::new("bad_key");
        client.base_url = server.base_url();

        let params = WebSearchQueryParamsBuilder::default().q("unauthorized").build().unwrap();

        let result = client.web_search(&params).await;

        assert!(result.is_err());
        // Assert that the error is a BraveClientError::HttpError
        if let Err(BraveClientError::HttpError(msg)) = result {
            assert!(msg.contains("401"));
        } else {
            panic!("Expected BraveClientError::HttpError");
        }
        mock.assert();
    }
}
