extern crate primal;

use primal::Primes;

// sqrt(600851475143) = 775146.099
const MAX_NUM: usize = 775147;
const NUMERATOR: usize = 600851475143;

fn main() {
    let mut largest_factor = 1;

    for prime in Primes::all() {
        if prime > MAX_NUM {
            break;
        }

        if NUMERATOR % prime == 0 {
            largest_factor = prime;
        }
    }

    println!("{}", largest_factor);
}
