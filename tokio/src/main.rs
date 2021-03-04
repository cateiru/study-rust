use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let handle = tokio::spawn(async {
        println!("AAA");

        "OK"
    });

    let out = handle.await.unwrap();

    println!("GOT {}", out);

    Ok(())
}
