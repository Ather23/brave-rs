# brave-rs

A Rust client library for the [Brave Search Web API](https://search.brave.com/api/docs/web-search/).  
Easily perform web searches and handle results in your Rust applications.

> **Note:** This project is under active development. The API and features may change.

## Features

- Async API using `reqwest` and `tokio`
- Query builder with support for all Brave Search parameters
- Strongly-typed responses using `serde`
- Custom error handling

## Usage

Add to your `Cargo.toml`:

```toml
brave-rs = { path = "." }
tokio = { version = "1", features = ["full"] }
```

Set your Brave API key as an environment variable:

**Command Prompt:**

```sh
set BRAVE_API_KEY=your_api_key_here
```

**PowerShell:**

```powershell
$env:BRAVE_API_KEY="your_api_key_here"
```

````Set your Brave API key as an environment variable:

**Command Prompt:**
```sh
set BRAVE_API_KEY=your_api_key_here
```

**PowerShell:**
```powershell
$env:BRAVE_API_KEY=
```

Example code:

```rust
use brave_rs::BraveClient;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("BRAVE_API_KEY").expect("BRAVE_API_KEY not set");
    let client = BraveClient::new(&api_key);
    let result = client.web_search_by_query("what is rust lang?").await;
    println!("{:#?}", result);
}
```

## TODO

- [ ] Add more API endpoints (images, news, videos)
- [ ] Improve documentation and examples
- [ ] Add integration tests
- [ ] Publish to crates.io
- [ ] Support for custom
````
