use std::cmp::max;
use std::mem::swap;

static TRIANGLE: &[&[i32]] = include!("p067_triangle.rs");

fn main() {
    let mut row = TRIANGLE.len() - 1;
    let mut values = TRIANGLE[row].to_vec();
    let mut next = Vec::with_capacity(TRIANGLE[row].len() - 1);

    while row > 0 {
        row -= 1;

        next.extend_from_slice(TRIANGLE[row]);

        for (i, parent) in next.iter_mut().enumerate() {
            *parent += max(values[i], values[i + 1]);
        }

        swap(&mut values, &mut next);
        next.clear();
    }

    println!("{}", values[0]);
}
