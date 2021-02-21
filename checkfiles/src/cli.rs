use std::process;

use clap::{App, Arg};

pub struct Cli {
  pub target_path: String,
}

impl Cli {
  pub fn new() -> Self {
    let matches = App::new("check files")
      .arg(
        Arg::with_name("target")
          .short("t")
          .long("target")
          .takes_value(true),
      )
      .get_matches();

    let target_path = matches.value_of("target");

    match target_path {
      Some(path) => Cli {
        target_path: path.to_string(),
      },
      None => {
        // if `--target` flag value is none, call error and exit.
        println!("Error: target is none.");
        process::exit(1);
      }
    }
  }

  // pub fn create_path(&self) -> Path {
  //   let path: &str = &self.target_path;
  //   Path::new(path)
  // }
}
