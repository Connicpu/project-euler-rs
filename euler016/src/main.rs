extern crate num;

use num::bigint::BigInt;
use num::pow::pow;

fn main() {
    let sum: usize = pow(BigInt::from(2), 1000)
        .to_string()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .sum();
    println!("{}", sum);
}
