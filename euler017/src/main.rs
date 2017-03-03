extern crate numtoa;
use numtoa::NumToA;

fn main() {
    let result: usize = (0..1001).map(word_length).sum();
    println!("{}", result);
}

fn word_length(n: usize) -> usize {
    match n {
        0 => 0,
        1...19 => ONES[n - 1].len(),
        20...99 => TENS[first_digit(n) - 2].len() + word_length(n % 10),
        100...999 if n % 100 == 0 => ONES[first_digit(n) - 1].len() + "hundred".len(),
        100...999 => ONES[first_digit(n) - 1].len() + "hundredand".len() + word_length(n % 100),
        1000 => "onethousand".len(),
        _ => unimplemented!(),
    }
}

fn first_digit(n: usize) -> usize {
    (first_digit_char(n) - b'0') as usize
}

fn first_digit_char(n: usize) -> u8 {
    let mut buf = [0u8; 20];
    let i = n.numtoa(10, &mut buf);
    buf[i]
}

static ONES: [&str; 19] = ["one",
                           "two",
                           "three",
                           "four",
                           "five",
                           "six",
                           "seven",
                           "eight",
                           "nine",
                           "ten",
                           "eleven",
                           "twelve",
                           "thirteen",
                           "fourteen",
                           "fifteen",
                           "sixteen",
                           "seventeen",
                           "eighteen",
                           "nineteen"];

static TENS: [&str; 8] = ["twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty",
                          "ninety"];
