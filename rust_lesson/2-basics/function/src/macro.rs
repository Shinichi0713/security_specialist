// マクロを定義する
macro_rules! log {
    ($x:expr) => { println!("{}", $x); }
}

fn main() {
    // マクロ名! でマクロを呼び出す
    log!("ABC...");
}

