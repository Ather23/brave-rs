use brave_rs::BraveClient;

#[tokio::main]
async fn main() {
    let client = BraveClient::new("BSAwSh9LS1ruUon58nXdbtU8ljmBRNa");
    let result = client.web_search("what is rust lang?").await;
    println!("{:#?}", result);
}
