use std::cmp::max;
use std::str;

static DATA: &[u8] = include_bytes!("num.txt");

fn main() {
    let mut greatest = 0;

    for window in DATA.windows(13) {
        let product: i64 = window.iter().map(|&b| (b - b'0') as i64).product();
        greatest = max(greatest, product);
    }

    println!("{}", greatest);
}
