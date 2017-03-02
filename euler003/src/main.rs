extern crate primal;
use primal::Sieve;

fn main() {
    println!("{:?}", Sieve::new(775147).factor(600851475143).unwrap().last());
}
