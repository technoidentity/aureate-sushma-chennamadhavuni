fn main() {
    let yr = 2000;

    if yr % 4 == 0 && yr % 100 != 0 || yr % 400 == 0 {
        println!("{} is a leap year", yr);
    } else {
        println!("{} is not a leap year", yr);
    }
}
