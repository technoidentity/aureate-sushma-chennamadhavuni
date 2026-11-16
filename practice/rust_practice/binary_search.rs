fn main() {
    let mut arr: [i32; 7] = [2, 3, 13, 9, 29, 11, 53];
    let found = 13;
    let mut idx = 0;

    arr.sort();
    println!("Sorted array {:?}", arr);

    let mut high = arr.len() - 1;
    let mut low = 0;
    let mut mid = (high + low) / 2;

    while low <= high {
        if arr[mid] < found {
            low = mid + 1;
        } else if arr[mid] > found {
            high = mid - 1;
        } else {
            println!("{}", arr[mid]);
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
        println!("Element {} found at index {}", found, idx);
    } else {
        println!("Element {} not found", found);
    }
}
