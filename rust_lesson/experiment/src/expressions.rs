
pub fn exec_match(){
    let numbers = vec![10, 20, 30];

    let first = numbers.get(0);

    match first{
        Some(val) => println!("最初の値は {} です", val),
        None => println!("リストは空でした"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 何もなければそのまま None
        Some(i) => Some(i + 1), // あれば 1 を足して再び Some で包む
    }
}

pub fn exec_match2(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}