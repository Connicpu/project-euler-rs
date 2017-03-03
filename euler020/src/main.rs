extern crate num;

use num::bigint::BigInt;

fn main() {
    let sum: usize = (2i32..101)
        .fold(BigInt::from(1), |a, i| a * BigInt::from(i))
        .to_string()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .sum();
    println!("{}", sum);
}
