use std::{thread, time};

mod frame;

fn main() {
    let mut f = frame::create_frame();
    let mut index: usize = 0;

    let height_size = frame::HEIGHT;
    let width_size = frame::WIDTH;
    let all_element_size = height_size * width_size;

    loop {
        frame::print_frame(&f);

        if index >= all_element_size {
            f = frame::all_clear(f);
            index = 0;
        }

        let width = index % width_size;
        let height = index / width_size;

        f[height][width] = true;

        sleep(10);
        clear();
        index += 1;
    }
}

fn sleep(time: u64) {
    let ten_millis = time::Duration::from_millis(time);

    thread::sleep(ten_millis);
}

fn clear() {
    print!("{}[2J", 27 as char);
}
