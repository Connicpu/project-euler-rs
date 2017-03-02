fn main() {
    let mut total = 0;

    let mut i0 = 0;
    let mut i1 = 1;

    loop {
        let fib = i0 + i1;
        if fib > 4_000_000 {
            break;
        }

        i0 = i1;
        i1 = fib;

        if fib & 1 == 0 {
            total += fib;
        }
    }

    println!("{}", total);
}
