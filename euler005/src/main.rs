fn main() {
    'outer: for i in 2520.. {
        for j in 1..21 {
            if i % j != 0 {
                continue 'outer;
            }
        }

        println!("{}", i);
        break;
    }
}
