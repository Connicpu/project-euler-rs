// I'm pretty sure it's less than that
const MAX_NUM: usize = 1_000_000;

fn main() {
    let mut prime_states = [true; MAX_NUM];
    let mut num_primes = 0;

    // Fill it out like a sieve of eratosthenese
    for i in 2..MAX_NUM {
        if !prime_states[i] {
            continue;
        }

        num_primes += 1;
        if num_primes == 10_001 {
            println!("{}", i);
            break;
        }

        let mut j = i * 2;
        while j < MAX_NUM {
            prime_states[j] = false;
            j += i;
        }
    }
}
