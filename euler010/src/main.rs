// I'm pretty sure it's less than that
const MAX_NUM: usize = 2_000_000;

fn main() {
    let mut prime_states = vec![true; MAX_NUM];
    let mut sum = 0;

    // Fill it out like a sieve of eratosthenese
    for i in 2..MAX_NUM {
        if !prime_states[i] {
            continue;
        }

        // i is prime
        sum += i;

        let mut j = i * 2;
        while j < MAX_NUM {
            prime_states[j] = false;
            j += i;
        }
    }

    println!("{}", sum);
}
