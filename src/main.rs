use brave_rs::BraveClient;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("BRAVE_API_KEY").expect("BRAVE_API_KEY not set");
    let client = BraveClient::new(&api_key);
    let result = client.web_search_by_query("stock market news today").await;
    println!("{:#?}", result);
}
