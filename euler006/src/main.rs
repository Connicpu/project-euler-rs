fn main() {
    let sum: i32 = (1..100 + 1).sum();
    let sum_of_sqrs: i32 = (1..100 + 1).map(|i| i * i).sum();

    println!("{}", (sum * sum) - sum_of_sqrs);
}
