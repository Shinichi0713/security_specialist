fn main(){
    let square = | x: i32|{
        x * x
    };
    println!("{}", square(9));

    let msg = String::from("Hello");	// クロージャー外変数msg
    let func = move || {			// 所有権をクロージャーに移動すること宣言
        println!("{}", msg);		// 参照したクロージャー外変数の所有権はクロージャーに移動
    };					// クロージャー終了時に所有者が不在となり解放される
    func();					// クロージャーを呼び出す

    let pertVal = factorial(10);
    println!("{}", pertVal)
}

fn factorial(x: i32) -> i32 { // 1. 戻り値の型 -> i32 を追加
    if x <= 1 {             // 2. ifの条件式から括弧を除去
        // return 1; と書いても良いが、慣習的にセミコロンなしで式を記述
        1 
    } else {
        x * factorial(x - 1) 
    }
}


macro_rules! log {
    ($x:expr) => { println!("{}", $x); }
}