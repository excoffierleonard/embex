use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let image = "";

    let image_analysis = analyze_image(client, image).await.unwrap();

    println!("{image_analysis:?}");
}

async fn analyze_image(client: Client, image: &str) -> Result<String, reqwest::Error> {
    let res = client
        .post("http://ollama.local/api/generate")
        .json(&serde_json::json!({
            "model":"llama3.2-vision",
            "prompt": "What is in this picture?",
            "stream": false,
            "images": [image]
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
