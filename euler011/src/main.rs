use std::cmp::max;

static GRID: [[i32; 20]; 20] = include!("grid.txt");

fn col(x: usize, y: usize) -> i32 {
    if y > 16 {
        return 0;
    }

    (0..4).map(|i| GRID[y + i][x]).product()
}

fn row(x: usize, y: usize) -> i32 {
    if x > 16 {
        return 0;
    }

    (0..4).map(|i| GRID[y][x + i]).product()
}

fn diag_lr(x: usize, y: usize) -> i32 {
    if x > 16 || y > 16 {
        return 0;
    }

    (0..4).map(|i| GRID[y + i][x + i]).product()
}

fn diag_rl(x: usize, y: usize) -> i32 {
    if x < 4 || y > 16 {
        return 0;
    }

    (0..4).map(|i| GRID[y + i][x - i]).product()
}

fn main() {
    let mut greatest = 0;
    for y in 0..20 {
        for x in 0..20 {
            greatest = max(greatest, col(x, y));
            greatest = max(greatest, row(x, y));
            greatest = max(greatest, diag_lr(x, y));
            greatest = max(greatest, diag_rl(x, y));
        }
    }

    println!("{}", greatest);
}
