// main.rs - CLI-driven Luhn algorithm implementation
//
// This crate provides a CLI-driven implementation of the Luhn algorithm.
//
// # Examples
//
// ```
// $ cargo run -- 4539 1488 0343 6467
// Valid
// $ cargo run -- 8273 1232 7352 0569
// Valid
// $ cargo run -- 2323 2005 7766 3554
// Valid
// $ cargo run -- 2323200577663554
// Valid
// $ cargo run -- 2323-2005-7766-3554
// Valid
// $ cargo run -- 2323-2005-7766-3555
// Invalid
// ```
//
// use luhn::is_valid;
// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         println!("Usage: {} <number>", args[0]);
//         return;
//     }
//     let number = &args[1];
//     if is_valid(number) {
//         println!("Valid");
//     } else {
//         println!("Invalid");
//     }
// }
// /// ```

use crate::lib::is_valid;
use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }
    let number = &args[1];
    if is_valid(number) {
        println!("Valid");
    } else {
        println!("Invalid");
    }
}
