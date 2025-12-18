macro_rules! create_vec {
    // パターン 1: 引数なしの場合
    () => {
        // mut は不要なので、単に Vec::new() を返す
        Vec::new()
    };

    // パターン 2: ゼロ個以上の要素がある場合（以前の定義）
    ( $( $x:expr ),* ) => {
        {
            // 要素を追加するので mut が必要
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


fn main() {
    // 整数型の空のベクトルであることを明示
    let empty_i32_vec: Vec<i32> = create_vec!(); 
    println!("Empty Vec (i32): {:?}", empty_i32_vec);
    
    // 文字列型の空のベクトルであることを明示
    let empty_string_vec: Vec<&str> = create_vec!(); 
    println!("Empty Vec (str): {:?}", empty_string_vec);
}