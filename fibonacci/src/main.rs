fn fibonacci(a: i128) -> i128 {
    if a == 0 {
        return 0;
    } else if a == 1 {
        return 1;
    }
    fibonacci(a - 1) + fibonacci(a - 2)
}

fn main() {
    let num: i128 = 100;

    println!("ans: {}", fibonacci(num));
}
