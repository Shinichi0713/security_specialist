enum Result {
    Success(String), // 成功メッセージを持つ
    Error(u32),      // エラーコードを持つ
}

let res = Result::Success(String::from("完了！"));

match res {
    Result::Success(msg) => println!("おめでとう: {}", msg), // msgを取り出せる
    Result::Error(code) => println!("エラーコード: {}", code), // codeを取り出せる
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 何もなければそのまま None
        Some(i) => Some(i + 1), // あれば 1 を足して再び Some で包む
    }
}

let five = Some(5);
let six = plus_one(five);      // Some(6)
let none = plus_one(None);     // None

fn main() {
    let numbers = vec![10, 20, 30];

    // .get() は Option<&i32> を返す
    let first = numbers.get(0); 

    match first {
        // 値がある場合：中身（val）を取り出して使う
        Some(val) => println!("最初の値は {} です", val),
        
        // 値がない場合：パニックせずに別の処理をする
        None => println!("リストは空でした"),
    }
}
