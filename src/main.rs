use embex::{App, Config};

#[tokio::main]
async fn main() {
    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");

    match app.process_image("image.png").await {
        Ok(_) => println!("Successfully processed and stored image"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
