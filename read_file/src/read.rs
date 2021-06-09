use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file() -> Result<(), Box<dyn std::error::Error>> {
    for result in BufReader::new(File::open("a.txt")?).lines() {
        let l = result?;
        println!("{}", l);
    }
    Ok(())
}
