use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchApiResponse {
    #[serde(rename = "type")]
    pub result_type: String,
    pub discussions: Option<SearchResults>,
    pub faq: Option<FaqResults>,
    pub infobox: Option<Infobox>,
    pub locations: Option<LocationResults>,
    pub news: Option<NewsResults>,
    pub query: Query,
    pub web: Option<SearchResults>,
    pub videos: Option<VideoResults>,
    pub mixed: Option<MixedResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
    #[serde(rename = "type")]
    pub result_type: String,
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub age: Option<String>,
    pub page_age: Option<String>,
    pub language: Option<String>,
    pub locations: Option<Vec<String>>,
    pub family_friendly: Option<bool>,
    pub profile: Option<Profile>,
    pub subpages: Option<Vec<Subpage>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub url: String,
    pub long_name: Option<String>,
    pub img: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subpage {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaqResults {
    #[serde(rename = "type")]
    pub result_type: String,
    pub results: Vec<FaqResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FaqResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub question: String,
    pub answer: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Infobox {
    #[serde(rename = "type")]
    pub result_type: String,
    pub position: u32,
    pub label: String,
    pub category: String,
    pub long_desc: String,
    pub thumbnail: Option<Thumbnail>,
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub src: String,
    pub original: Option<String>,
    pub logo: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResults {
    #[serde(rename = "type")]
    pub result_type: String,
    pub results: Vec<LocationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub coordinates: Option<[f64; 2]>,
    pub postal_address: Option<PostalAddress>,
    pub contact: Option<Contact>,
    pub rating: Option<Rating>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    pub country: String,
    pub region: String,
    pub locality: String,
    pub street_address: String,
    pub postal_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub email: Option<String>,
    pub telephone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "ratingValue")]
    pub rating_value: f32,
    #[serde(rename = "bestRating")]
    pub best_rating: f32,
    #[serde(rename = "reviewCount")]
    pub review_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixedResponse {
    #[serde(rename = "type")]
    pub result_type: String,
    pub main: Option<Vec<ResultReferenceResult>>,
    pub top: Option<Vec<ResultReferenceResult>>,
    pub side: Option<Vec<ResultReferenceResult>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultReferenceResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub main: Option<Vec<MixedMain>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixedMain {
    #[serde(rename = "type")]
    pub result_type: String,
    pub index: u64,
    pub all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResults {
    #[serde(rename = "type")]
    pub result_type: String,
    pub results: Vec<NewsResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResult {
    pub url: String,
    pub title: String,
    pub description: String,
    pub age: String,
    pub page_age: Option<String>,
    pub breaking: Option<bool>,
    pub profile: Option<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub original: String,
    pub show_strict_warning: bool,
    pub is_navigational: bool,
    pub is_news_breaking: bool,
    pub spellcheck_off: bool,
    pub country: String,
    pub bad_results: bool,
    pub should_fallback: bool,
    pub postal_code: String,
    pub city: String,
    pub header_country: String,
    pub more_results_available: bool,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResults {
    #[serde(rename = "type")]
    pub result_type: String,
    pub results: Option<Vec<VideoResult>>,
    pub mutated_by_goggles: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub fetched_content_timestamp: Option<u64>,
    pub age: Option<String>,
    pub page_age: Option<String>,
    pub video: Option<VideoData>,
    pub meta_url: Option<MetaUrl>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaUrl {
    pub scheme: String,
    pub netloc: String,
    pub hostname: String,
    pub favicon: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoData {
    pub duration: Option<String>,
    pub views: Option<usize>,
    pub creator: Option<String>,
    pub publisher: Option<String>,
    pub thumbnail: Option<Thumbnail>,
    pub tags: Option<Vec<String>>,
    pub author: Option<Profile>,
    pub requires_subscription: Option<bool>,
}
