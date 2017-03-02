use std::cmp::max;

static TRIANGLE: &[&[i32]] = include!("p067_triangle.rs");

fn main() {
    let mut row = TRIANGLE.len() - 1;
    let mut values = TRIANGLE[row].to_vec();

    while row > 0 {
        row -= 1;

        let row = TRIANGLE[row];
        for i in 0..row.len() {
            let bigger = max(values[i], values[i + 1]);
            values[i] = row[i] + bigger;
        }
    }

    println!("{}", values[0]);
}
