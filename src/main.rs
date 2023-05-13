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

use std::env;

fn main() {
    // this simple independent command line utility can both generate card numbers with `luhn generate` and check if a given number is valid with `luhn check`
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args.len() != 2 {
        println!("Usage: {} <generate|check> <number>", args[0]);
        return;
    }
    let command = &args[1];
    let number = &args[2];
    match command.as_str() {
        "generate" if args.len() == 2 => {
            println!("{}", luhn::generate());
        }
        "check" if args.len() == 3 => {
            if luhn::is_valid(number) {
                println!("Valid");
            } else {
                println!("Invalid");
            }
        }

        _ => {
            println!("Usage: {} <generate|check> <number>", args[0]);
            return;
        }
    }
}
