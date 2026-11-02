fn main() {
    let mut n = 60;
    let mut arr = Vec::with_capacity(100);
    let mut i = 2;

    while i <= n {
        if is_prime(i) && n % i == 0 {
            arr.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }

    println!("{:?}", arr);
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    let mut count = 0;

    for i in 1..n + 1 {
        if n % i == 0 {
            count += 1;
        }

        if count > 2 {
            return false;
        }
    }

    count == 2
}
