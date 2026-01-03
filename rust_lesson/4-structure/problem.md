Rustの構造体を実際にコーディングして理解を深めるための、実践的な例題を作成しました。

今回のテーマは、**「図書館の蔵書管理システム」**です。データの集約、カプセル化（情報の隠蔽）、そしてメソッドの実装を体験できる内容になっています。

---

## 課題：図書館の蔵書管理システム

以下の仕様を満たすように、構造体 `Book` を実装してください。

### 1. 構造体の定義

`Book` という名前の構造体を作成し、以下のフィールドを持たせてください。

* `title` (String): 本のタイトル
* `author` (String): 著者名
* `pages` (u32): ページ数
* `is_borrowed` (bool): 貸出中かどうか（初期値は `false`）

### 2. メソッドの実装 (`impl` ブロック)

以下の3つの関数・メソッドを実装してください。

1. **関連関数 `new**`:
* タイトル、著者名、ページ数を受け取り、新しい `Book` インスタンスを返します。
* `is_borrowed` は自動的に `false` に設定します。


2. **メソッド `borrow_book**`:
* 本を貸し出します。`is_borrowed` を `true` に書き換えます。
* すでに貸出中の場合は、「すでに貸出中です」と表示してください。


3. **メソッド `return_book**`:
* 本を返却します。`is_borrowed` を `false` に書き換えます。



---

## 雛形コード（ここを埋めてください）

```rust
struct Book {
    // 1. ここにフィールドを定義してください
}

impl Book {
    // 2. 関連関数 new を実装してください
    fn new(title: &str, author: &str, pages: u32) -> Self {
        // ここに処理を書く
    }

    // 3. メソッド borrow_book を実装してください
    fn borrow_book(&mut self) {
        // ここに処理を書く
    }

    // 4. メソッド return_book を実装してください
    fn return_book(&mut self) {
        // ここに処理を書く
    }
}

fn main() {
    // テストコード
    let mut my_book = Book::new("Rustプログラミング入門", "フェリス", 300);
    
    println!("本を借ります...");
    my_book.borrow_book();
    
    println!("もう一度借りてみます...");
    my_book.borrow_book(); // ここで警告が出るはず
    
    println!("本を返します...");
    my_book.return_book();
}

```

---

## ヒント

* 自分自身のデータを書き換えるメソッドには、引数に `&mut self` が必要です。
* 文字列を受け取る際は、`&str` で受け取って `.to_string()` または `.into()` で `String` 型に変換するのが一般的です。


