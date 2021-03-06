mod utils;

fn main() {
    let max_size = 100;
    let sleep_time = 30;

    for i in 0..=max_size {
        let ratio = ((i as f64 / max_size as f64) * 100f64) as usize;
        let space: String = {
            match ratio as usize {
                n if n < 10 => String::from("  "),
                100 => String::from(""),
                _ => String::from(" "),
            }
        };
        let bar = utils::create_bar(ratio as f64 / 100f64, max_size);
        utils::print_data(format!("{:}%{} |{}|", ratio, space, bar));
        utils::sleep(sleep_time);
    }
}
