Rustで「辞書」のような、**キーと値をペアにして保存するデータ構造**は、標準ライブラリの **`HashMap<K, V>`** を使って定義します。

他言語の `dict` (Python) や `Object` (JavaScript), `Map` (Java/C++) に相当するものです。

---

## 🏗️ HashMap の基本的な定義と使い方

`HashMap` を使うには、まず標準ライブラリからインポートする必要があります。

**Rust**

```
use std::collections::HashMap;

fn main() {
    // 1. 新しい空の辞書を作成
    // 型は HashMap<キーの型, 値の型> となります
    let mut scores = HashMap::new();

    // 2. データの挿入 (Insert)
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. 値の取得 (Get)
    let team_name = String::from("Blue");
    // get は Option<&V> を返すので、安全に処理します
    match scores.get(&team_name) {
        Some(score) => println!("{team_name}のスコア: {score}"),
        None => println!("チームが見つかりません"),
    }

    // 4. データの削除 (Remove)
    scores.remove("Yellow");

    // 5. すべての要素をループで処理
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

---

## 🔑 HashMap の仕組み

`HashMap` は内部で **ハッシュ関数** を使って、キーをメモリ上のインデックスに変換しています。

* **高速な検索:** インデックスを直接計算するため、データが大量にあっても特定のキーをほぼ一瞬（平均 **$O(1)$**）で見つけることができます。
* **順序の不定:** ベクトルとは違い、データを入れた順番は保持されません。

---

## 💡 知っておくと便利なテクニック

### 1. まとめて初期化する

ベクトルには `vec!` マクロがありますが、`HashMap` には公式の標準マクロがありません。しかし、配列やベクトルから変換（`collect`）することでスマートに初期化できます。

**Rust**

```
use std::collections::HashMap;

fn main() {
    // タプルの配列から HashMap を生成
    let data = [("Apple", 100), ("Banana", 200)];
    let fruit_prices: HashMap<_, _> = data.into_iter().collect();
}
```

### 2. 「値がなければ挿入する」

辞書でよくある「すでにキーがあれば何もしない、なければ初期値をいれる」という処理は、`entry` API を使うと1行で書けます。

**Rust**

```
// "Green"チームがいなければ、初期値 0 をセット
scores.entry(String::from("Green")).or_insert(0);
```

---

## 📊 ベクトル (Vec) と 辞書 (HashMap) の使い分け

| **特徴**         | **ベクトル (Vec`<T>`)** | **辞書 (HashMap<K, V>)** |
| ---------------------- | ------------------------------- | ------------------------------ |
| **アクセス方法** | 番号（0, 1, 2...）              | 自由なキー（名前、IDなど）     |
| **順序**         | 保持される                      | **保持されない**         |
| **主な用途**     | 順番に並んだリスト              | 特定のラベルで値を検索したい時 |

次は、この辞書データを使って「文章の中の単語の数を数える」ような実践的な例を見てみますか？
