extern crate primal;
use primal::StreamingSieve;

fn main() {
    println!("{}", StreamingSieve::nth_prime(10_001));
}
