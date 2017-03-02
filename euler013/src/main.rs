extern crate num;

use num::bigint::BigInt;

static NUMBERS: [&[u8]; 100] = include!("data.txt");

fn main() {
    let mut sum: BigInt = Default::default();

    for &num in &NUMBERS[..] {
        sum = sum + BigInt::parse_bytes(num, 10).unwrap();
    }

    println!("{}", &sum.to_string()[..10]);
}
