use std::io;

fn main() {
    let arr: [i32; 7] = [2, 3, 9, 11, 13, 29, 53];
    let mut idx = 0;

    println!("Array is: {:?}", arr);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let found: i32 = input.trim().parse().unwrap();

    let mut high = arr.len() - 1;
    let mut low = 0;
    let mut mid = (high + low) / 2;

    while low <= high {
        if arr[mid] < found {
            low = mid + 1;
        } else if arr[mid] > found {
            high = mid - 1;
        } else {
            idx = mid;
            break;
        }

        mid = (high + low) / 2;
    }

    let mut is_found = false;

    if arr[idx] == found {
        is_found = true;
    }

    if is_found {
        println!("Element found at index {}", idx);
    } else {
        println!("Element not found");
    }
}
