//! lib.rs - implementation of the Luhn algorithm
//!
//! This crate provides a function to check if a given number is valid according to the Luhn algorithm.
//!
//! # Examples
//!
//! ```
//! use luhn::is_valid;
//!
//! assert!(is_valid("4539 1488 0343 6467"));
//! assert!(is_valid("8273 1232 7352 0569"));
//! assert!(is_valid("2323 2005 7766 3554"));
//! assert!(is_valid("2323200577663554"));
//! assert!(is_valid("2323-2005-7766-3554"));
//! ```

use rand::Rng;

/// Check if a given number is valid according to the Luhn algorithm.
///
/// # Examples
///
/// ```
/// use luhn::is_valid;
///
/// assert!(is_valid("4539 1488 0343 6467"));   // Visa
/// assert!(is_valid("8273 1232 7352 0569"));   // Diners Club
/// assert!(is_valid("2323 2005 7766 3554"));   // MasterCard
/// assert!(is_valid("2323200577663554"));      // MasterCard
/// assert!(is_valid("2323-2005-7766-3554"));   // MasterCard
/// ```
pub fn is_valid(number: &String) -> bool {
    let mut sum = 0;
    if number.len() < 2 {
        return false;
    }

    let mut parity = number.len() % 2 == 0;
    for c in number.chars().filter(|c| !c.is_whitespace()) {
        match c.to_digit(10) {
            Some(digit) => {
                if parity {
                    sum += digit;
                } else {
                    sum += if digit < 5 { digit * 2 } else { digit * 2 - 9 };
                }
                parity = !parity;
            }
            None => return false,
        }
    }
    sum % 10 == 0
}

/// Generate a random valid number according to the Luhn algorithm.
/// Requires the `rand` feature.
///
/// # Examples
///
/// ```
/// use luhn::generate;
///
/// let number = generate();
/// assert!(luhn::is_valid(number));
/// ```
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut number = String::new();
    while !crate::is_valid(&number) {
        number = rng.gen_range(0..0xDE0B6B3A7640000u64)
            .to_string()
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| if i % 2 == 0 { c.to_digit(10).unwrap() } else { c.to_digit(10).unwrap() * 2 })
            .map(|c: u32 | if c > 57 { c - 9 } else { c })
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
    }
    number
}