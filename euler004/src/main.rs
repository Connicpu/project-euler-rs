extern crate numtoa;

use numtoa::NumToA;
use std::cmp::max;

fn is_palindrome(num: i32) -> bool {
    let mut buf = [0u8; 16];
    let start = num.numtoa(10, &mut buf[..]);
    let count = 16 - start;

    for i in 0..count / 2 {
        if buf[start + i] != buf[start + count - i - 1] {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut largest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindrome(i * j) {
                largest = max(largest, i * j);
            }
        }
    }

    println!("{}", largest);
}
