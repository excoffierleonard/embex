use embex::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    match app.process_image("image.png").await {
        Ok(response) => println!("Analysis result: {response}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
