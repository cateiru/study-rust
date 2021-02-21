mod cli;

use crate::cli::Cli;

fn main() {
    let cli = Cli::new();

    println!("{}", cli.target())
}
