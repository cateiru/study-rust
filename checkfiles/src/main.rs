use std::{fs, path::Path, process};

mod cli;

use crate::cli::Cli;

fn main() {
    let cli = Cli::new();

    let target: &Path = cli.target();

    // match target.to_str() {
    //     Some(a) => println!("{}", a),
    //     _ => process::exit(1),
    // }

    let a = fs::read_dir(target);

    for e in a {
        println!("{:?}", e);
    }
}
