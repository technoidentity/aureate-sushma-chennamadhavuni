use std::io;

fn main(){

    let arr=[1,2,7,9,15,21,33,37,43,47,49,53,69,76,83,88,91,100];
    let mut high=arr.len()-1;
    let mut low=0;

    println!("{:?}", arr);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let target: i32 = input.trim().parse().unwrap();

    while low<=high{
        let mid=(low+high)/2;
        if arr[mid]==target{
            println!("The number is found at index {}",mid);
            break;
        }
        else if arr[mid]<target{
            low=mid+1;
        }
        else{
            if mid == 0 {
                println!("The number was not found");
                break;
            }
            high=mid-1;
        }
    }
    if low > high {
        println!("The number was not found");
    }
}
