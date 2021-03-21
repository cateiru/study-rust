use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let a: Option<String> = None;

  if a.is_none() {
    println!("Ok");
  }
  Ok(())
}
