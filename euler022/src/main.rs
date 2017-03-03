fn score(i: usize, name: &str) -> usize {
    name.bytes().map(|b| (b - b'A' + 1) as usize).sum::<usize>() * (i + 1)
}

fn main() {
    let mut names = include!("p022_names.rs");
    names.sort();

    let sum: usize = names.iter()
        .enumerate()
        .map(|(i, name)| score(i, name))
        .sum();

    println!("{}", sum);
}
