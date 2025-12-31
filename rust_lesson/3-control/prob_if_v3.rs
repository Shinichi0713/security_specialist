
use std::io;

fn is_adult(age: u32) -> bool{
    if age >= 20{
        true
    } else{
        false
    }
}


fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();
    let result = is_adult(age);
    println!("The age {} is {}", age, result);
}
