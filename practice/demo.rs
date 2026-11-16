fn main() {
    // STRING INTERPOLATION
    let greetings = "Hello";
    let subject = "World";
    println!("{} {}", greetings, subject);

    // FLOAT DATA TYPE
    let x = 1.2;
    let y = 2.2;
    println!("{}", x * y);

    // MUTABILITY
    let mut x = 5;
    x = 10;
    println!("{}", x);

    // FUNCTION
    let answer = multiply_both(1.1, 2.2);
    println!("1.1 × 2.2 = {}", answer);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    x * y
}
