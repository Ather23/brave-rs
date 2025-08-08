use crate::types::WebSearchQueryParams;

impl From<WebSearchQueryParams> for String {
    fn from(params: WebSearchQueryParams) -> Self {
        web_search_query_builder(&params).unwrap_or_default()
    }
}

pub fn web_search_query_builder(params: &WebSearchQueryParams) -> Option<String> {
    let mut params_vec = vec![];

    if let Some(q) = &params.q {
        params_vec.push(format!("q={}", urlencoding::encode(q)));
    }
    if let Some(count) = params.count {
        params_vec.push(format!("count={}", count));
    }
    if let Some(offset) = params.offset {
        params_vec.push(format!("offset={}", offset));
    }
    if let Some(search_lang) = &params.search_lang {
        params_vec.push(format!("search_lang={}", urlencoding::encode(search_lang)));
    }
    if let Some(safesearch) = &params.safesearch {
        params_vec.push(format!("safesearch={}", urlencoding::encode(safesearch)));
    }
    if let Some(freshness) = &params.freshness {
        params_vec.push(format!("freshness={}", urlencoding::encode(freshness)));
    }
    if let Some(country) = &params.country {
        params_vec.push(format!("country={}", urlencoding::encode(country)));
    }
    if let Some(source) = &params.source {
        params_vec.push(format!("source={}", urlencoding::encode(source)));
    }

    if params_vec.is_empty() {
        None
    } else {
        Some(format!("/web/search?{}", params_vec.join("&")))
    }
}
