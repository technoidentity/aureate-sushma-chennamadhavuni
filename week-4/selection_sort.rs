fn main(){
    let mut arr=[1,3,19,71,63,2,47,29,84,17];
    println!("{:?}", arr);
    for i in 0..arr.len(){
        let mut min_index=i;
        for j in (i+1)..arr.len(){
            if arr[j]<arr[min_index]{
                min_index=j;
            }
        }
        arr.swap(i, min_index);
    }
    println!("sorted array: {:?}", arr);
}