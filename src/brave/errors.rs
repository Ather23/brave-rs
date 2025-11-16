#[derive(Debug, thiserror::Error)]
pub enum BraveClientError {
    #[error("Client error: {0}")] ClientError(String),
    #[error("Http error: {0}")] HttpError(String),
    #[error("Unable to deserialize response")] ResponseDeserializationError(String),
}

impl From<reqwest::Error> for BraveClientError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_status() {
            if let Some(status) = err.status() {
                return BraveClientError::HttpError(format!("HTTP error {}: {}", status, err));
            }
        }
        BraveClientError::ClientError(err.to_string())
    }
}
