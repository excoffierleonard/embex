#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("http://ollama.local/api/tags")
        .await?
        .text()
        .await?;

    println!("{body:?}");
    Ok(())
}
