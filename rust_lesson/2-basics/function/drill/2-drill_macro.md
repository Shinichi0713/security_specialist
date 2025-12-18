## Rust マクロの例題

### 1. `say_hello!` マクロ：単純なコードの繰り返しを避ける

最も基本的な例として、特定の文字列をコンソールに出力するマクロを作成します。これは、関数では引数を取らないと難しい、特定のパターンのコード生成の例です。

**Rust**

```
// 宣言的マクロの定義
macro_rules! say_hello {
    // マッチングルール: 引数なしの場合
    () => {
        // 展開されるコード
        println!("Hello, Rustaceans!");
    };
}

fn main() {
    // マクロの呼び出し
    say_hello!();
    say_hello!(); // 何度でも簡単に呼び出せる
}
```

**出力:**

**Plaintext**

```
Hello, Rustaceans!
Hello, Rustaceans!
```

---

### 3. `calculate!` マクロ：異なるパターンによるコード生成

異なる数の引数や異なる構造に合わせて、異なるコードを生成する例です。これは、マクロが引数のパターンによって動作を切り替えられることを示しています。

**Rust**

```
macro_rules! calculate {
    // パターン 1: 2つの引数を受け取り、足し算を行う
    ( $a:expr, $b:expr ) => {
        $a + $b
    };

    // パターン 2: 3つの引数を受け取り、掛け算を行う
    ( $a:expr, $b:expr, $c:expr ) => {
        $a * $b * $c
    };

    // パターン 3: 引数が一つで、それを二乗する
    ( $a:expr ) => {
        $a * $a
    };
}

fn main() {
    let sum = calculate!(10, 5);         // パターン 1 がマッチ
    let product = calculate!(2, 3, 4);   // パターン 2 がマッチ
    let square = calculate!(7);          // パターン 3 がマッチ

    println!("Sum (10 + 5): {}", sum);
    println!("Product (2 * 3 * 4): {}", product);
    println!("Square (7 * 7): {}", square);
}
```

**出力:**

**Plaintext**

```
Sum (10 + 5): 15
Product (2 * 3 * 4): 24
Square (7 * 7): 49
```

---

これらの例は、Rustの**宣言的マクロ (`macro_rules!`)** が、引数の数や形に応じて異なるコードを生成し、コードの繰り返しを減らしたり、より慣用的なAPI（例: `vec!`）を提供したりするのに役立つことを示しています。

他に、特定のマクロのパターンについて詳しく知りたいことはありますか？
