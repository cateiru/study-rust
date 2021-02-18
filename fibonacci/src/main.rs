// https://www.christopherbiscardi.com/why-can-t-i-early-return-in-an-if-statement-in-rust

// TYPE: 1
// fn fibonacci(a: u128) -> u128 {
//     if a <= 1 {
//         a
//     } else {
//         fibonacci(a - 1) + fibonacci(a - 2)
//     }
// }

// TYPE: 2
// fn fibonacci(a: u128) -> u128 {
//     match a {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(a - 1) + fibonacci(a - 2),
//     }
// }

fn fibonacci(a: u128) -> u128 {
    println!("{}", a);
    if a == 10 {
        1
    } else {
        0
    };

    0
}

fn main() {
    let num: u128 = 10;

    println!("ans: {}", fibonacci(num));
}
