// fn main() {
//     println!("Hello, World!");
// }
// BASIC HELLO WORLD PROGRAM

// fn reverse_string(s: &str) -> String {
//     s.chars().rev().collect()
// }

// fn main() {
//     let original = "rust programming";
//     let reversed = reverse_string(original);

//     println!("{reversed}");
// }
// REVERSE STRING

// fn main() {
//     let n = 5;
//     let mut sum=0;
//     for i in 1..=n{
//         sum+=i*i;
//     }
//     println!("{}", sum-n*(n+1)/2);
// }
// ARMSTRONG NUMBER

// fn main(){
//     let yr=2000;
//     if yr % 4 == 0 && yr % 100 != 0 || yr % 400 == 0 {
//         println!("{} is a leap year", yr);
//     } else {
//         println!("{} is not a leap year", yr);
//     }
// }
//LEAP YEAR

// fn main() {
//     let mut n = 60;
//     let mut arr = Vec::with_capacity(100);
//     let mut i = 2;
//     while i <= n {
//         if is_prime(i) && n % i == 0 {
//             arr.push(i);
//             n /= i;
//         } else {
//             i += 1;
//         }
//     }
//     println!("{:?}", arr);
// }

// fn is_prime(n: i32) -> bool {
//     if n <= 1 {
//         return false;
//     }
//     let mut count = 0;
//     for i in 1..n + 1 {
//         if n % i == 0 {
//             count += 1;
//         }
//         if count > 2 {
//             return false;
//         }
//     }
//     count == 2
// }
// PRIME FACTORS OF A GIVEN NUMBER

// use std::collections::HashSet;
// fn main() {
//     let mut numbers = HashSet::new();
//     let x:i32=3;
//     let y:i32=5;
//     let z:i32=20;
//     for i in 1..=z/2{
//         if x*i < z{
//          numbers.insert(x*i);
//         }
//         if y*i < z {
//             numbers.insert(y*i);
//         }
//     }
//     println!("{:?}", numbers);
//     println!("{}", numbers.iter().sum::<i32>());
// }
// SUM OF ALL UNIQUE MULTIPLES OF 3 OR 5 BELOW 20

// fn main(){
//     let mut arr:[i32;7]=[2,3,13,9,29,11,53];
//     let found=13;
//     let mut idx=0;

//     arr.sort();
//     println!("Sorted array {:?}", arr);

//     let mut high=arr.len()-1;
//     let mut low=0;
//     let mut mid=(high+low)/2;

//     while low <= high{
//         if arr[mid]<found{
//             low=mid+1;
//         }
//         else if arr[mid]>found{
//             high=mid-1;
//         }
//         else {
//             println!("{}", arr[mid]);
//             idx=mid;
//             break;
//         }
//         mid=(high+low)/2;
//     }
//     let mut is_found = false;
//     if arr[idx] == found {
//         is_found = true;
//     }
//     if is_found {
//         println!("Element {} found at index {}", found, idx);
//     } else {
//         println!("Element {} not found", found);
//     }
// }
// BINARY SEARCH

// use std::io;
// fn main(){
//     let arr:[i32;7]=[2,3,9,11,13,29,53];
//     let mut idx=0;

//     println!("Array is: {:?}", arr);
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let found: i32 = input.trim().parse().unwrap();


//     let mut high=arr.len()-1;
//     let mut low=0;
//     let mut mid=(high+low)/2;


//     while low <= high{
//         if arr[mid]<found{
//             low=mid+1;
//         }
//         else if arr[mid]>found{
//             high=mid-1;
//         }
//         else {
//             idx=mid;
//             break;
//         }
//         mid=(high+low)/2;
//     }
//     let mut is_found = false;
//     if arr[idx] == found {
//         is_found = true;
//     }
//     if is_found {
//         println!("Element found at index {}", idx);
//     } else {
//         println!("Element not found");
//     }
// }
// BINARY SEARCH USING STANDARD IO
