#![feature(conservative_impl_trait)]

extern crate primal;

use primal::Sieve;
use std::collections::HashSet;

/// Lists all of the multiples of a factor of n which are also
/// factors of n.
fn factor_multiples(p: usize, n: usize) -> impl Iterator<Item = usize> {
    (1..)
        .map(move |i| p * i)
        .take_while(move |&f| f < n)
        .filter(move |&f| n % f == 0)
}

/// Gets the sum of all factors of a number given its prime factorization
fn factor_sum(f: Vec<(usize, usize)>, n: usize) -> usize {
    f.iter()
        .cloned()
        .flat_map(|t| factor_multiples(t.0, n))
        .chain(Some(1))
        .collect::<HashSet<_>>() // simple O(n) unsorted dedup
        .iter()
        .cloned()
        .sum()
}

/// Checks if a number is abundant by
fn is_abundant(n: usize, sieve: &Sieve) -> bool {
    sieve.factor(n)
        .map(|f| factor_sum(f, n) > n)
        .unwrap_or(false)
}

const MAX: usize = 28123;

fn main() {
    let sieve = Sieve::new(MAX);

    // Find all of the abundant numbers less than MAX
    let mut abundant = HashSet::new();
    for n in 1..MAX {
        if is_abundant(n, &sieve) {
            abundant.insert(n);
        }
    }

    // Helper for checking if a number cannot be represented
    // as a sum of two abundant numbers
    let not_abundant_sum = |n| {
        !abundant.iter()
            .any(|&f| abundant.contains(&(n - f)))
    };

    // Sum of all of such numbers
    let sum: usize = (1..MAX)
        .filter(|&n| not_abundant_sum(n))
        .sum();

    println!("{}", sum);
}
