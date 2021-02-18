use std::env;

fn main() -> std::io::Result<()> {
  let a = env::current_dir()?;

  println!("{}", a.display());
  Ok(())
}
