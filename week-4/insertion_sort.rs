fn main(){
    let mut arr=[1,22,3,19,71,36,2,47,29,84,17];
    println!("{:?}", arr);

     for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    println!("sorted array : {:?}", arr);
}
