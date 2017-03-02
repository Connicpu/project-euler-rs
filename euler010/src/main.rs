extern crate primal;
use primal::Primes;

fn main() {
    println!("{}", Primes::all().take_while(|&p| p < 2_000_000).sum::<usize>());
}
