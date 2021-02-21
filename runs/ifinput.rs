fn main() {
  let a: Option<String> = Some("Hello".to_string());
  // let a: Option<String> = None;

  // ここでbに譲渡。aは再呼び出し不可になる。
  if let Some(b) = a {
    println!("OK {}", b);
  }

  println!("{:?}", a); // Error: value borrowed here after partial move
}
