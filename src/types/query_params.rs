use derive_builder::Builder;

#[derive(Debug, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct WebSearchQueryParams {
    pub q: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
    pub search_lang: Option<String>,
    pub safesearch: Option<String>,
    pub freshness: Option<String>,
    pub country: Option<String>,
    pub source: Option<String>,
}
