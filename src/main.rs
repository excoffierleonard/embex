use reqwest::Client;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let image = "";

    match analyze_image(client, image).await {
        Ok(response) => println!("{response}"),
        Err(e) => println!("Error: {e}"),
    }
}

async fn analyze_image(client: Client, image: &str) -> Result<String, reqwest::Error> {
    let response = client
        .post("http://ollama.local/api/generate")
        .json(&json!(
            {
                "model":"llama3.2-vision",
                "prompt": "What is in this picture?",
                "stream": false,
                "images": [image]
            }
        ))
        .send()
        .await?;

    let json_response: Value = response.json().await?;
    Ok(json_response["response"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}
