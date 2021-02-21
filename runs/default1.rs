struct SomeOptions {
  foo: i32,
  bar: f32,
}

impl SomeOptions {
  fn new() -> Self {
    Self { foo: 0, bar: 0.0f32 }
  }

  fn call(&self) -> String {
    format!("{0}, {1}, Hello", self.bar, self.foo)
  }
}

fn main() {
  let options: SomeOptions = SomeOptions::new();

  println!("{}", options.call());
}
