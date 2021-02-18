use std::env;
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
  let a = env::current_dir()?;

  println!("{}", a.display());
  Ok(())
}
