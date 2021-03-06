mod utils;

fn main() {
    let max_size = 100;
    let sleep_time = 50;

    // for i in 0..=max_size {
    //     utils::progress_bar(i, max_size, sleep_time);
    // }

    let mut i = 0;

    while i <= max_size {
        utils::progress_bar(i, max_size, sleep_time);
        i = match i {
            50 => random_select(51, 30),
            70 => random_select(71, 65),
            85 => random_select(86, 80),
            86 => random_select(87, 60),
            90 => random_select(91, 87),
            95 => random_select(96, 94),
            _ => i + 1,
        }
    }
}

fn random_select(i: usize, other_i: usize) -> usize {
    let mut new_i = i;
    if rand::random() {
        new_i = other_i
    }

    new_i
}
