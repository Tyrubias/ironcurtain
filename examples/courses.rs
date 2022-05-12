use std::error::Error;

use ironcurtain::Canvas;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = std::env::var("CANVAS_API_TOKEN")?;

    let canvas = Canvas::builder()
        .set_url("https://canvas.rice.edu")
        .set_token(&token)
        .build()?;

    let my_courses = canvas.courses().my_courses().await?;

    dbg!(my_courses);

    Ok(())
}
