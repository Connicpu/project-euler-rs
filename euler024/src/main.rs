fn do_permute(values: &mut Vec<char>, found: &mut usize) -> bool {
    if values.is_empty() {
        *found += 1;
        if *found == 1_000_000 {
            values.clear();
            return true;
        }
    }

    for i in 0..values.len() {
        let c = values.remove(i);
        if do_permute(values, found) {
            values.push(c);
            return true;
        } else {
            values.insert(i, c);
        }
    }

    false
}

fn main() {
    let mut numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut found = 0;
    do_permute(&mut numbers, &mut found);

    numbers.reverse();
    for c in numbers {
        print!("{}", c);
    }
    println!("");
}
