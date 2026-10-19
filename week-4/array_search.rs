fn main() {
    let arr = [1, 22, 3, 19, 71, 36, 2, 47, 29, 84, 17];
    // let input = 10;
    let input = 36;

    let mut found = false;
    for i in 0..arr.len() {
        if arr[i] == input {
            println!("{input} found at index {i}");
            found = true;
            break;
        }
    }

    if !found {
        println!("{input} was not found");
    }
}
