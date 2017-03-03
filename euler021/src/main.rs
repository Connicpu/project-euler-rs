#![feature(conservative_impl_trait)]

extern crate primal;

use primal::Sieve;
use std::collections::{HashMap, HashSet};

/// Lists all of the multiples of a factor of n which are also
/// factors of n.
fn factor_multiples(p: usize, n: usize) -> impl Iterator<Item = usize> {
    (1..)
        .map(move |i| p * i)
        .take_while(move |&f| f < n)
        .filter(move |&f| n % f == 0)
}

/// Gets the sum of all factors of a number given its prime factorization
fn get_factor_sum(f: Vec<(usize, usize)>, n: usize) -> usize {
    f.iter()
        .cloned()
        .flat_map(|t| factor_multiples(t.0, n))
        .chain(Some(1))
        .collect::<HashSet<_>>() // simple O(n) unsorted dedup
        .iter()
        .cloned()
        .sum()
}

fn factor_sum(n: usize, sieve: &Sieve) -> usize {
    let factors = sieve.factor(n).unwrap();
    get_factor_sum(factors, n)
}

fn main() {
    let sieve = Sieve::new(10_000);

    let mut factor_sums = HashMap::new();
    for i in 1..10_000 {
        factor_sums.insert(i, factor_sum(i, &sieve));
    }

    let mut amicable = HashSet::new();
    for (a, b) in &factor_sums {
        if factor_sums.get(b) == Some(a) && a != b {
            amicable.insert(*a);
            amicable.insert(*b);
        }
    }

    println!("{}", amicable.iter().cloned().sum::<usize>());
}
