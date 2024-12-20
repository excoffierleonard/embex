use embex::{App, Config};

#[tokio::main]
async fn main() {
    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config);

    match app.process_image("image.png").await {
        Ok(response) => println!("Analysis result: {response}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
