fn main() {
    let n = 5;
    let mut sum = 0;

    for i in 1..=n {
        sum += i * i;
    }

    println!("{}", sum - n * (n + 1) / 2);
}
