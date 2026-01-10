今回はRustの **`match` 式** につちえ説明します。

Rustの`match`は、値のパターンを照合し、一致したパターンに応じたコードを実行する非常に強力な制御構造です。
他言語の `switch` 文に似ていますが、より安全で、表現力が高いのが特徴です。


###  `match` 式の基本構造

`match` は「値」を受け取り、上から順番に「パターン」をチェックしていきます。

```rust
let number = 3;

match number {
    1 => println!("1です"),
    2 => println!("2です"),
    3 => println!("3です"), // ここで一致！
    _ => println!("それ以外です"), // ワイルドカード（その他すべて）
}

```

### 主な特徴
主な特徴を以下に説明します。


#### ① 網羅性チェック (Exhaustiveness)

Rustコンパイラは、 **「すべての可能性が考慮されているか」** を厳格にチェックします。もし1つでもパターンを書き忘れると、コンパイルエラーになります。これにより、バグを未然に防ぐことができます。

#### ② データの取り出し (Destructuring)

`enum`（列挙型）と組み合わせることで、中に包まれているデータを取り出すことができます。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

match msg {
    Message::Quit => println!("終了"),
    Message::Move { x, y } => println!("x: {}, y: {} へ移動", x, y),
    Message::Write(text) => println!("メッセージ: {}", text),
}

```

#### ③ 「式」として値を返す

`match` は「文」ではなく「式」です。そのため、結果を変数に代入することができます。

```rust
let is_allowed = match user_role {
    "admin" => true,
    "editor" => true,
    _ => false,
};

```


### 3. `match` のパターン色々

| パターン | 書き方 | 説明 |
| --- | --- | --- |
| **リテラル** | `1`, `"hello"` | 特定の値に一致させる |
| **範囲** | `1..=5` | 1から5までの値に一致させる |
| **複数一致** | `1 | 2` |
| **ワイルドカード** | `_` | 何にでも一致（defaultに相当） |
| **ガード条件** | `n if n > 10` | 一致した上で、さらに条件を満たすか判定 |


## ユースケース
Rustの `match` 式が特に「便利だ！」と感じる瞬間は、単なる条件分岐を超えて、 **「データの構造を分解しながら、安全性を担保したい時」** です。

具体的に、他の言語の `if` や `switch` よりも圧倒的に便利な4つのシーンを紹介します。


### 1. Enum（列挙型）からデータを取り出す時

これが最大のメリットです。RustのEnumは中にデータを持てるため、`match` を使うことで「どの種類か」の判定と「中のデータを取り出す（バインド）」ことを同時に行えます。

```rust
enum Result {
    Success(String), // 成功メッセージを持つ
    Error(u32),      // エラーコードを持つ
}

let res = Result::Success(String::from("完了！"));

match res {
    Result::Success(msg) => println!("おめでとう: {}", msg), // msgを取り出せる
    Result::Error(code) => println!("エラーコード: {}", code), // codeを取り出せる
}

```

### 2. 「考慮漏れ」をコンパイラに指摘してほしい時

新しい機能を追加して、Enumの選択肢が増えた場合、`match` を使っていればコンパイラが「新しいパターンが処理されていません」とエラーを出してくれます。

* **便利さ**: 修正が必要な箇所をコンパイラがすべて教えてくれるため、大規模な開発でもデバッグが劇的に楽になります。

### 3. 複雑な条件を「ガード」でスッキリ書きたい時

`match` のパターンには `if` 条件（マッチガード）を付けることができます。ネストした `if` 文を書かずに済むため、コードが非常に読みやすくなります。

```rust
let pair = (2, -2);

match pair {
    (x, y) if x == y => println!("同じ値です"),
    (x, y) if x + y == 0 => println!("反対の値です"),
    (x, _) if x % 2 == 0 => println!("xは偶数です"),
    _ => println!("それ以外"),
}

```

### 4. 値をそのまま変数に代入したい時

`match` は「式」なので、値を返します。これにより、初期化と条件分岐を1行で行えます。

```rust
let message = match status_code {
    200 => "OK",
    404 => "Not Found",
    500 => "Internal Server Error",
    _ => "Unknown Error",
}; // セミコロンで終わる

```


### 比較：if と match の使い分け

| 特徴 | `if / else` | `match` |
| --- | --- | --- |
| **得意なこと** | 真偽値（true/false）の判定 | 複数の状態やデータの分解 |
| **網羅性チェック** | なし（書き忘れても動く） | **あり（書き忘れるとコンパイル不可）** |
| **可読性** | 条件が2つまでならスッキリ | 3つ以上の分岐なら圧倒的に綺麗 |

**「状態が3つ以上ある」「データの中身を取り出したい」「一箇所も漏れなく処理したい」** 。このどれかに当てはまるなら、迷わず `match` を使うのがRust流の正解です。

## 実装法

Rustにおいて、値が「ある（`Some`）」か「ない（`None`）」かを表す `Option<T>` 型を処理する際、`match` 式は最も基本的かつ安全な方法です。

なぜなら、 **「値がない場合（None）」の処理を書き忘れるとコンパイラが許してくれない** からです。


### 1. `match` を使った基本的な処理

以下は、リストから最初の要素を取得し、その結果（`Option`）を安全に処理する例です。

```rust
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

```


### 2. 計算結果の `Option` を処理する

数値の入った `Option` に対して、値があるときだけ計算を行い、その結果を新しい変数に代入する例です。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 何もなければそのまま None
        Some(i) => Some(i + 1), // あれば 1 を足して再び Some で包む
    }
}

let five = Some(5);
let six = plus_one(five);      // Some(6)
let none = plus_one(None);     // None

```


### 3. より簡潔に書きたい時：`if let`

「値がある時だけ何かをしたい（`None` の時は何もしなくていい）」という場合は、`match` よりも **`if let`** が便利です。

```rust
let some_value = Some(100);

// 「もし Some(i) にマッチするなら」という書き方
if let Some(i) = some_value {
    println!("値は {} でした", i);
}
// None の場合の else は省略可能

```


### 4. メソッドを使った「モダン」な処理

`match` を書くほどでもない単純なケースでは、`Option` が持つ便利なメソッドを使うのが Rust 流です。

* **`.unwrap_or(default)`**: 値があれば取り出し、なければデフォルト値を返す。
* **`.map(f)`**: 値があれば関数 `f` を適用する。

```rust
let x: Option<i32> = None;

// 値がなければ 0 を使う（match を書くより一行で済む）
let value = x.unwrap_or(0); 

```



