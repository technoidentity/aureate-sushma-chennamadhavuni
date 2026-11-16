fn main() {
    let n: u32 = 1531;
    let digits = n.to_string().len() as u32;

    let mut temp = n;
    let mut sum: u32 = 0;

    while temp > 0 {
        let digit = temp % 10;
        sum += digit.pow(digits);
        temp /= 10;
    }

    if sum == n {
        println!("{} is an Armstrong number", n);
    } else {
        println!("{} is not an Armstrong number", n);
    }
}

