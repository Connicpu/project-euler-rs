use std::collections::HashMap;

fn find_chain_len(mut n: usize, chains: &HashMap<usize, usize>) -> usize {
    let mut len = 0;
    loop {
        if let Some(chain) = chains.get(&n) {
            return len + chain;
        }

        len += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }
}

fn main() {
    let mut chains = HashMap::new();
    let mut greatest = 0;
    let mut greatest_n = 0;

    chains.insert(1, 1);

    for n in 1..1_000_000 {
        let len = find_chain_len(n, &chains);
        chains.insert(n, len);
        if len > greatest {
            greatest = len;
            greatest_n = n;
        }
    }

    println!("{}", greatest_n);
}
