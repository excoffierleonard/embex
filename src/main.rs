#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let body = client
        .get("http://ollama.local/api/tags")
        .send()
        .await?
        .text()
        .await?;

    println!("{body:?}");
    Ok(())
}
