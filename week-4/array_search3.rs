fn main() {
    let arr = [1, 22, 3, 19, 71, 36, 2, 47, 29, 84, 17];
    let index = 20;

    if index < arr.len() {
        println!("Value at index {index} is {}", arr[index]);
    } else {
        println!("Index not found");
    }
}
