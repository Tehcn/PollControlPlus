use reqwest;

async fn vote() {
    let client = reqwest::Client::new();
    client.post("http://httpbin.org/post")
        .body()
        .send()
        .await;
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    for _ in 0..100 {
        vote().await;
    }
}