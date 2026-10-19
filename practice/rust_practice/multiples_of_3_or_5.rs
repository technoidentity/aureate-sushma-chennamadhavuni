use std::collections::HashSet;

fn main() {
    let mut numbers = HashSet::new();
    let x: i32 = 3;
    let y: i32 = 5;
    let z: i32 = 20;

    for i in 1..=z / 2 {
        if x * i < z {
            numbers.insert(x * i);
        }

        if y * i < z {
            numbers.insert(y * i);
        }
    }

    println!("{:?}", numbers);
    println!("{}", numbers.iter().sum::<i32>());
}
