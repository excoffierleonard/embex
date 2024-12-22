use embex::{App, Config};

#[tokio::main]
async fn main() {
    let config = Config::build().expect("Failed to load configuration");
    let app = App::new(config).await.expect("Failed to initialize app");

    match app
        .process_image(vec![
            "tests/resources/test_image_1.png",
            "tests/resources/test_image_2.png",
        ])
        .await
    {
        Ok(_) => println!("Successfully processed and stored image"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
