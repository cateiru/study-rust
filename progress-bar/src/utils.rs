use std::{
  io::{stdout, Write},
  thread, time,
};

pub fn sleep(time: u64) {
  let ten_millis = time::Duration::from_millis(time);

  thread::sleep(ten_millis);
}

pub fn print_data(text: String) {
  print!("\r{}", text);
  stdout().flush().unwrap();
}

pub fn create_bar(ratio: f64, width: usize) -> String {
  let fill = (ratio * width as f64) as usize;
  let mut bar: String = String::from("");

  for i in 0..width {
    if fill > i {
      bar.push_str("#");
    } else {
      bar.push_str(" ");
    }
  }
  bar
}
