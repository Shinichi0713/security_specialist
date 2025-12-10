macro_rules! print_vars{
    ($($e:expr), *) =>{
        $(
            println!("{}:{:?}", stringify!($e), $e);
        )*
    }
}


fn main(){
    let a = 10;
    let b = 5;
    let name = "Alice";

    println!("--- print_vars! マクロの実行 ---");
    print_vars!(a, b, a*b, name.len(), name);
    println!("--------------------------------");
    
    // マクロはブロックや関数呼び出しも展開できます
    let result = {
        let x = 20;
        x + 3
    };
    print_vars!(result, result - 1);
}


/*
1. パターンマッチング (($($e:expr),*))
$e:expr: マッチングルールの一つで、「$e という名前で式にマッチさせる」という意味です。

$($e:expr),*: これは、$e:expr がコンマ (,) で区切られてゼロ回以上繰り返されるパターンにマッチします。

2. メタプログラミングの機能
$e: マッチした式そのものが、展開後のコードにそのまま挿入され、Rustのコンパイル時に評価されます。

stringify!($e): マクロ専用の機能で、式 $e を評価するのではなく、その式の文字列表現（つまりソースコードに書かれたとおりの変数名や式）に変換します。

例: stringify!(a * b) は "a * b" という文字列になります。
*/

