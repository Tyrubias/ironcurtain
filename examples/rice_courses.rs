use ironcurtain::Client;

#[tokio::main]
async fn main() {
    let token = std::env::var("CANVAS_API_TOKEN").unwrap();

    let client = Client::builder()
        .set_url("canvas.rice.edu".to_string())
        .set_token(token)
        .build();

    let response = client.get_courses().await;

    match response {
        Ok(courses) => println!("{:?}", courses),
        Err(error) => eprintln!("Failed to get courses: {:?}", error),
    }
}
