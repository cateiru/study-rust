use std::{path::Path, process};

use clap::{App, Arg};

pub struct Cli {
  target_path: String,
  file_path: String,
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
      .arg(
        Arg::with_name("file")
          .short("f")
          .long("file")
          .takes_value(true),
      )
      .get_matches();

    let target_path = check_args(matches.value_of("target"));
    let file_path = check_args(matches.value_of("file"));

    Cli {
      target_path: target_path.to_string(),
      file_path: target_path.to_string(),
    }
  }

  /// Return the target path.
  /// parse in the Path.
  pub fn target(&self) -> &Path {
    Path::new(&self.target_path)
  }
}

/// Check Optional element.
/// if it is none, throw error.
fn check_args<T>(element: Option<T>) -> T {
  match element {
    Some(path) => path,
    None => {
      println!("Error");
      process::exit(1);
    }
  }
}
