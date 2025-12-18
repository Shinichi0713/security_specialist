### 2. `create_vec!` マクロ：初期化を簡略化する（標準ライブラリの簡略版）

要素をいくつか受け取り、それを格納した `Vec`（ベクトル）を生成するマクロを作成します。これは、`vec!`マクロの簡単な再現であり、マクロがコンマ区切りのリストを扱う方法を示しています。

**Rust**

```
// 宣言的マクロの定義
macro_rules! create_vec {
    // マッチングルール: ゼロ個以上の要素 (expr) をコンマで区切って受け取る
    // $() で繰り返しを指定し、* でゼロ回以上の繰り返しを示します。
    // , で繰り返し要素間の区切り文字を指定します。
    ( $( $x:expr ),* ) => {
        {
            // 展開されるコード
            let mut temp_vec = Vec::new();

            // それぞれの要素に対して繰り返し処理を行う
            $(
                temp_vec.push($x);
            )*

            temp_vec // 最後にベクトルを返す
        }
    };
}

fn main() {
    // マクロの呼び出し
    let numbers = create_vec!(10, 20, 30, 40);
    let fruits = create_vec!("apple", "banana", "cherry");
    let empty_vec = create_vec!();

    println!("Numbers: {:?}", numbers);
    println!("Fruits: {:?}", fruits);
    println!("Empty Vec: {:?}", empty_vec);
}
```

**出力:**

**Plaintext**

```
Numbers: [10, 20, 30, 40]
Fruits: ["apple", "banana", "cherry"]
Empty Vec: []
```
