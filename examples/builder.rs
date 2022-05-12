use ironcurtain::Canvas;

#[tokio::main]
pub async fn main() {
    let token = std::env::var("CANVAS_API_TOKEN").unwrap();

    let canvas = Canvas::builder()
        .set_token(&token)
        .set_url("https://canvas.rice.edu/")
        .build();

    match canvas {
        Ok(canvas) => println!("{:?}", canvas),
        Err(error) => eprintln!("Error while building Canvas: {}", error),
    }
}
