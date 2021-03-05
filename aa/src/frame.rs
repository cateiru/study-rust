pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 9;

pub fn create_frame() -> Vec<Vec<bool>> {
  let mut frame = Vec::with_capacity(HEIGHT);

  for _ in 0..HEIGHT {
    frame.push(vec![false; WIDTH]);
  }

  frame
}

pub fn all_clear(mut frame: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  for height in 0..HEIGHT {
    for width in 0..WIDTH {
      frame[height as usize][width as usize] = false;
    }
  }

  frame
}

pub fn print_frame(frame: &Vec<Vec<bool>>) {
  for height in 0..HEIGHT {
    for width in 0..WIDTH {
      if frame[height as usize][width as usize] {
        print!("■ ");
      } else {
        print!("□ ");
      }
    }
    print!("\n");
  }
}
