use std::io;

fn judge_leap_year(year: i32) -> &'static str{
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0){
        "Leap Year"
    } else {
        "Not a Leap Year"
    }
}


fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let year: i32 = input.trim().parse().unwrap();
    let result = judge_leap_year(year);
    println!("The year {} is {}", year, result);
}