fn fac_recursive(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * fac_recursive(n - 1)
    }
}

fn main() {
    println!("{}", fac_recursive(2));
}
