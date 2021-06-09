use read_file::read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    read::file()?;
    Ok(())
}
