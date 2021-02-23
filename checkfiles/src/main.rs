use std::{error::Error, fs, io, path::Path, process};

mod cli;

use crate::cli::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::new();

    let target: &Path = cli.target();

    // match target.to_str() {
    //     Some(a) => println!("{}", a),
    //     _ => process::exit(1),
    // }

    // for path in fs::read_dir(target)? {
    //     let entry = path?;
    //     let path = entry.path();

    //     if path.is_dir() {
    //         println!("{:?} is directory.", path);
    //     } else {
    //         println!("{:?} is file.", path);
    //     }
    // }

    files(target, "".to_string())?;

    Ok(())
}

fn files(dir: &Path, indent: String) -> io::Result<()> {
    if dir.is_dir() {
        let paths: Vec<_> = fs::read_dir(dir)
            .unwrap()
            .map(|res| res.unwrap().path())
            .collect();

        let all_files = paths.len();
        let mut new_indent;

        for (index, path) in paths.iter().enumerate() {
            if path.is_dir() {
                if all_files - 1 == index {
                    println!("{}└─ {:?}", indent, path);
                    new_indent = format!("{}  ", indent);
                } else {
                    println!("{}├─ {:?}", indent, path);
                    new_indent = format!("{}│ ", indent);
                }

                files(&path, new_indent)?;
            } else {
                if all_files - 1 == index {
                    println!("{}└─ {:?}", indent, path);
                } else {
                    println!("{}├─ {:?}", indent, path);
                }
            }
        }
    }

    Ok(())
}
