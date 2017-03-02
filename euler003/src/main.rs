// sqrt(600851475143) = 775146.099
const MAX_NUM: usize = 775147;
const NUMERATOR: usize = 600851475143;

fn main() {
    let mut prime_states = [true; MAX_NUM];
    let mut largest_factor = 1;

    // Fill it out like a sieve of eratosthenese
    for i in 2..MAX_NUM {
        if !prime_states[i] {
            continue;
        }

        let mut j = i * 2;
        while j < MAX_NUM {
            prime_states[j] = false;
            j += i;
        }

        if NUMERATOR % i == 0 {
            largest_factor = i;
        }
    }

    println!("{}", largest_factor);
}
