
今回のテーマは、**「汎用的な統計解析ツール」**の作成です。

### 課題1：汎用的な「座標」構造体と「距離」計算メソッド

以下の仕様を満たすように、ジェネリクスを用いたプログラムを実装してください。

#### 1. 構造体の定義

`Point<T>` という名前の構造体を作成してください。

* フィールドは `x` と `y` の2つで、どちらもジェネリックな型 `T` を持ちます。

#### 2. メソッドの実装 (`impl` ブロック)

この `Point<T>` に対して、以下のメソッドを実装してください。

1. **関連関数 `new**`:
* `x` と `y` を受け取り、新しい `Point<T>` インスタンスを返します。


2. **メソッド `mix_values**`:
* 2つの座標値を入れ替えた、新しい `Point<T>` を返します。（例：`(x: 10, y: 20)` なら `(x: 20, y: 10)` を返す）
* ※このメソッドは、どんな型 `T` でも動作するようにしてください。



#### 3. トレイト境界（発展）

`f64`（浮動小数点数）などの数値型だけで動作する、**原点 (0, 0) からの距離**を計算するメソッド `distance_from_origin` を追加してみてください。

* **ヒント**: 数値計算（累乗など）を行うには、`T` が特定の性質を持っている必要がありますが、今回はまず **`impl Point<f64> { ... }`** と記述して、`f64` 専用のメソッドとして定義してみましょう。


#### 雛形コード

```rust
// 1. ジェネリックな構造体 Point を定義してください
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 2. new メソッドを実装してください
    fn new(x: T, y: T) -> Self {
        // ここに処理を書く
    }

    // 3. x と y を入れ替えた新しい Point を返すメソッドを実装してください
    fn swap_xy(self) -> Point<T> {
        // ここに処理を書く
    }
}

// 4. f64 型の時だけ使えるメソッドを定義（具体的な型を指定する impl）
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        // 原点からの距離 = sqrt(x^2 + y^2)
        // ヒント: self.x.powi(2) や self.x.sqrt() を使います
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // 整数型の Point
    let p1 = Point::new(10, 20);
    let p1_swapped = p1.swap_xy();
    println!("p1 swapped: x={}, y={}", p1_swapped.x, p1_swapped.y);

    // 浮動小数点型の Point
    let p2 = Point::new(3.0, 4.0);
    println!("p2 distance: {}", p2.distance_from_origin());
}

```


### この問題のポイント

* **`<T>` の位置**: `impl` ブロックでジェネリクスを使う場合、`impl<T> Point<T>` のように `impl` の直後にも `<T>` を書く必要があります。
* **特定の型への限定**: `impl Point<f64>` と書くことで、「すべての `Point` ではなく、`f64` を使っている時だけこの関数を出現させる」という出し分けができます。


### 課題：有効期限付きデータラッパー `Expired<T>`

以下の仕様を満たすように実装してください。

#### 1. 構造体の定義

`Expired<T>` という名前の構造体を作成してください。

* `data`: 型 `T`（ジェネリックなデータ）
* `id`: `u32`（識別子）

#### 2. メソッドの実装 (`impl` ブロック)

全ての `T` に対して動作する以下のメソッドを実装してください。

1. **関連関数 `new**`:
* `id` と `data` を受け取り、インスタンスを返します。


2. **メソッド `get_data**`:
* データの参照（`&T`）を返します。


3. **メソッド `replace_data**`:
* 新しいデータ（型 `T`）を受け取り、古いデータを入れ替えて、**古い方のデータ**を返します。


#### 雛形コード

```rust
// 1. ジェネリックな構造体 Expired を定義してください
struct Expired<T> {
    data: T,
    id: u32,
}

impl<T> Expired<T> {
    // 2. new メソッドを実装してください
    fn new(id: u32, data: T) -> Self {
        // ここに記述
    }

    // 3. データの参照を返すメソッド
    fn get_data(&self) -> &T {
        // ここに記述
    }

    // 4. データを入れ替えて古いデータを返すメソッド
    fn replace_data(&mut self, new_data: T) -> T {
        // ヒント: std::mem::replace を使うか、一時変数に退避させます
        std::mem::replace(&mut self.data, new_data)
    }
}

fn main() {
    // ケース1: String型を包む
    let mut secret = Expired::new(1, String::from("旧パスワード"));
    println!("現在のデータ: {}", secret.get_data());

    let old_data = secret.replace_data(String::from("新パスワード"));
    println!("入れ替え完了。古いデータ: '{}' を破棄しました。", old_data);

    // ケース2: 数値(i32)を包む
    let price = Expired::new(100, 5000);
    println!("商品ID: {}, 価格: {}", price.id, price.get_data());
}

```


### この問題のポイント

* **参照を返す**: `get_data` で `&T` を返すことで、所有権を奪わずに中身を覗き見ることができます。
* **値の入れ替え**: `replace_data` では、ジェネリックな型 `T` のサイズが不明でも安全に入れ替えができる `std::mem::replace` の使いどころを学べます。

### なぜこれが「良いジェネリクス」なのか？

もしジェネリクスを使わなければ、`ExpiredString`、`ExpiredI32`、`ExpiredUser` といった構造体を大量に作る羽目になります。この `Expired<T>` が一つあれば、**将来あなたが作成するどんな新しい構造体に対しても、「期限管理機能」を即座に付与できる**ようになります。



## 問題2

今回のテーマは、 **「ジェネリックなスタック（Stack）構造体」** の作成です。
スタックは「後入れ先出し（LIFO）」のデータ構造ですが、中身が数値でも文字列でも動作するようにジェネリクスを使って実装してみましょう。


### 💻 実装課題：汎用スタック `MyStack<T>`

以下の仕様と雛形コードに従って、プログラムを完成させてください。

#### 1. 構造体の定義

`MyStack<T>` という名前の構造体を定義してください。

* 内部にデータを保持するためのフィールド `items` を持ちます。
* `items` の型は `Vec<T>` としてください。

#### 2. メソッドの実装 (`impl<T> MyStack<T>`)

以下の 4 つのメソッドを実装してください。

1. **`new`**: 空のスタックを生成して返す。
2. **`push`**: 引数で受け取ったデータ（型 `T`）を `items` の末尾に追加する。
3. **`pop`**: `items` の末尾からデータを取り出して返す。
* 戻り値の型は **`Option<T>`** にしてください（スタックが空の場合は `None` を返すため）。


4. **`peek`**: スタックの末尾にあるデータの**参照**を返す。
* 戻り値の型は **`Option<&T>`** にしてください。



### 📝 雛形コード

このコードをエディタにコピーして、`todo!()` の部分を書き換えてください。

```rust
// 1. 構造体の定義
struct MyStack<T> {
    items: Vec<T>,
}

// 2. ジェネリックなメソッドの実装
impl<T> MyStack<T> {
    // 新しいスタックを作成
    fn new() -> Self {
        todo!("Vec::new() を使って初期化してください")
    }

    // データを追加
    fn push(&mut self, item: T) {
        todo!("self.items に追加してください")
    }

    // データを取り出す（一番上の値を削除して返す）
    fn pop(&mut self) -> Option<T> {
        todo!("Vec の pop メソッドを利用してください")
    }

    // 一番上の値を覗き見する（削除しない）
    fn peek(&self) -> Option<&T> {
        todo!("Vec の last メソッドを利用してください")
    }
}

fn main() {
    // 数値型のスタックとして利用
    let mut int_stack = MyStack::new();
    int_stack.push(10);
    int_stack.push(20);
    println!("Peek int: {:?}", int_stack.peek()); // Some(20)
    println!("Pop int: {:?}", int_stack.pop());   // Some(20)

    // 文字列型のスタックとして利用
    let mut str_stack = MyStack::new();
    str_stack.push(String::from("Rust"));
    str_stack.push(String::from("Generics"));
    println!("Peek str: {:?}", str_stack.peek()); // Some("Generics")
}

```


### 💡 ヒント

* **`Vec<T>` の活用**: Rustの標準ライブラリにある `Vec` 自体がジェネリックなので、そのメソッド（`push`, `pop`, `last`）をそのまま利用すればOKです。
* **所有権の意識**: `peek` メソッドではデータを「取り出す」のではなく「覗き見」するだけなので、所有権を奪わないように `&T`（参照）を返すのがポイントです。

実装が完了したら、コードを教えてください。正しく実装できているか確認させていただきます！

**この問題が解けたら、次は「特定の条件（トレイト境界）をつけたジェネリクス」に挑戦してみますか？**
