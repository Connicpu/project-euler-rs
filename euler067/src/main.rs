use std::cmp::max;

static TRIANGLE: &[&[i32]] = include!("p067_triangle.rs");

fn main() {
    let mut row = TRIANGLE.len() - 1;
    let mut values = TRIANGLE[row].to_vec();

    while row > 0 {
        row -= 1;

        assert!(TRIANGLE[row].len() < values.len());
        for i in 0..TRIANGLE[row].len() {
            let bigger = max(values[i], values[i + 1]);
            values[i] = TRIANGLE[row][i] + bigger;
        }

        values.pop();
    }

    println!("{}", values[0]);
}
