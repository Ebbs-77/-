#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("This is my own app.");
    let mut sum = 0;
    for i in 1..=10 {
        sum += i;
    }
    println!("sum of 1..10 = {}", sum);
    0
}
