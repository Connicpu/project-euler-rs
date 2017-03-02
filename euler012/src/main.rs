fn main() {
    let mut tri = 0;
    for i in 1.. {
        tri += i;

        let mut factors = 0;
        for i in 1..(tri as f32).sqrt() as i32 {
            if tri % i == 0 {
                factors += 2;

                if factors > 500 {
                    println!("=== {}", tri);
                    return;
                }
            }
        }
    }
}
