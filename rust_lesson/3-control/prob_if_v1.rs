
fn judge_number(num: i32) -> &'static str{
    if num >0 {
        "Positive"
    } else if num < 0 {
        "Negative"
    } else{
        "Zero"
    }
}


fn main(){
    let number = 10;
    let result = judge_number(number);
    println!("The number {} is {}", number, result);
}

