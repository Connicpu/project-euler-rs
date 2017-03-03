#![feature(i128_type)]

fn fac_ratio(n: i128, k: i128) -> i128 {
    (k + 1..n + 1).fold(1, |i, a| i * a)
}

fn fac(n: i128) -> i128 {
    fac_ratio(n, 1)
}

fn main() {
    let n = 20;

    println!("{}", fac_ratio(n + n, n) / fac(n));
}
