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
        let response = self.get_request_builder(&query).send().await?;
        let json = response.text().await.unwrap();
        let result = serde_json::from_str::<WebSearchApiResponse>(&json);
        match result {
            Ok(resp) => {
                return Ok(resp);
            }
            Err(err) => Err(BraveClientError::ResponseDeserializationError(err.to_string())),
        }
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

    #[tokio::test]
    async fn test_web_search_video_result() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET).path("/web/search").query_param("q", "rust video");
            then.status(200)
                .header("Content-Type", "application/json")
                .json_body(
                    serde_json::json!({
                        "type": "videos",
                        "query": {
                            "original": "rust video",
                            "show_strict_warning": false,
                            "is_navigational": false,
                            "is_news_breaking": false,
                            "spellcheck_off": false,
                            "country": "us",
                            "bad_results": false,
                            "should_fallback": false,
                            "postal_code": "",
                            "city": "",
                            "header_country": "",
                            "more_results_available": false,
                            "state": ""
                        },
                        "videos": {
                            "type": "videos",
                            "results": [
                                {
                                    "type": "video_result",
                                    "url": "https://www.youtube.com/watch?v=5C_HPTJg5ek&pp=0gcJCfwAo7VqN5tD",
                                    "title": "Rust in 100 Seconds",
                                    "description": "Enjoy the videos and music you love, upload original content, and share it all with friends, family, and the world on YouTube.",
                                    "fetched_content_timestamp": 1755933539,
                                    "video": {},
                                    "meta_url": {
                                        "scheme": "https",
                                        "netloc": "youtube.com",
                                        "hostname": "www.youtube.com",
                                        "favicon": "https://imgs.search.brave.com/Wg4wjE5SHAargkzePU3eSLmWgVz84BEZk1SjSglJK_U/rs:fit:32:32:1:0/g:ce/aHR0cDovL2Zhdmlj/b25zLnNlYXJjaC5i/cmF2ZS5jb20vaWNv/bnMvOTkyZTZiMWU3/YzU3Nzc5YjExYzUy/N2VhZTIxOWNlYjM5/ZGVjN2MyZDY4Nzdh/ZDYzMTYxNmI5N2Rk/Y2Q3N2FkNy93d3cu/eW91dHViZS5jb20v",
                                        "path": "› watch"
                                    },
                                    "thumbnail": {
                                        "src": "https://imgs.search.brave.com/TECNOdC1xY5QK9FrXS28Ut5jocySHkFKcGthOxWpulo/rs:fit:200:200:1:0/g:ce/aHR0cHM6Ly9pLnl0/aW1nLmNvbS92aS81/Q19IUFRKZzVlay9o/cWRlZmF1bHQuanBn",
                                        "original": "https://i.ytimg.com/vi/5C_HPTJg5ek/hqdefault.jpg"
                                    }
                                }
                            ],
                            "mutated_by_goggles": false
                        }
                    })
                );
        });

        let mut client = BraveClient::new("test_key");
        client.base_url = server.base_url();

        let result = client.web_search_by_query("rust video").await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.result_type, "videos");
        let videos = response.videos.unwrap();
        let video_result = videos.results.unwrap();

        assert_eq!(videos.result_type, "videos");
        assert_eq!(video_result.len(), 1);
        let video_result = &video_result[0];
        assert_eq!(video_result.title, "Rust in 100 Seconds");
        assert_eq!(
            video_result.url,
            "https://www.youtube.com/watch?v=5C_HPTJg5ek&pp=0gcJCfwAo7VqN5tD"
        );
        mock.assert();
    }
    #[tokio::test]
    async fn test_web_search_with_mixed_field() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/web/search").query_param("q", "mixed");
            then.status(200)
                .header("Content-Type", "application/json")
                .json_body(
                    serde_json::json!({
                        "type": "web",
                        "query": {
                            "original": "mixed",
                            "show_strict_warning": false,
                            "is_navigational": false,
                            "is_news_breaking": false,
                            "spellcheck_off": false,
                            "country": "us",
                            "bad_results": false,
                            "should_fallback": false,
                            "postal_code": "",
                            "city": "",
                            "header_country": "",
                            "more_results_available": false,
                            "state": ""
                        },
                        "mixed": {
                            "type": "mixed",
                            "main": [
                                {
                                    "type": "reference",
                                    "main": [
                                        {"type": "main", "index": 1, "all": true}
                                    ]
                                }
                            ],
                            "top": null,
                            "side": null
                        }
                    })
                );
        });
        let mut client = BraveClient::new("test_key");
        client.base_url = server.base_url();
        let result = client.web_search_by_query("mixed").await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.mixed.is_some());
        let mixed = response.mixed.as_ref().unwrap();
        assert_eq!(mixed.result_type, "mixed");
        assert!(mixed.main.is_some());
        let main = mixed.main.as_ref().unwrap();
        assert_eq!(main.len(), 1);
        let ref_result = &main[0];
        assert_eq!(ref_result.result_type, "reference");
        assert!(ref_result.main.is_some());
        let mixed_main = ref_result.main.as_ref().unwrap();
        assert_eq!(mixed_main.len(), 1);
        assert_eq!(mixed_main[0].result_type, "main");
        assert_eq!(mixed_main[0].index, 1);
        assert!(mixed_main[0].all);
        mock.assert();
    }
}
