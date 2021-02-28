use serde_json::json;
use serde_json::Value;
use std::ffi::OsStr;
use std::{env, error, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
  let out_dir = env::var_os("OUT_DIR").expect("No OUT_DIR variable.");

  generate_languages(&out_dir)?;

  let sample_out_path = Path::new("sample.txt");
  std::fs::write(sample_out_path, format!("{:?}", out_dir))?;
  Ok(())
}

fn generate_languages(out_dir: &OsStr) -> Result<(), Box<dyn error::Error>> {
  let mut tera = tera::Tera::default();

  let json: Value = json!({
    "name": "hoge"
  });

  let output_path = Path::new(&out_dir).join("hello.rs");
  let rust_code = tera.render_str(
    &std::fs::read_to_string("src/hello.tera.rs")?,
    &tera::Context::from_value(json)?,
  )?;
  std::fs::write(output_path, rust_code)?;

  Ok(())
}
