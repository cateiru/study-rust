use reqwest::get;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = get("https://www.rust-lang.org").await?.text().await?;

    println!("body = {:?}", body);

    Ok(())
}
