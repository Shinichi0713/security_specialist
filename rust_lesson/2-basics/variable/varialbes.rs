
const MAX_ATTEMPTS: u32 = 5; // 定数はすべて大文字で記述するのが慣習
const PI: f64 = 3.14159265359;

fn main() {
    immutable_and_mutable_variables();
    shadowing_variables();
    constants();
    ownership_and_borrowing();
}

fn immutable_and_mutable_variables() {
    println!("--- 1. 不変変数と可変変数 ---");

    // 不変変数 (デフォルト)
    let x = 5;
    println!("不変変数 x: {}", x);

    // x = 6; // コンパイルエラー: `x` は不変のため再代入できません
    // error[E0384]: cannot assign twice to immutable variable `x`

    // 可変変数
    let mut y = 10;
    println!("可変変数 y (初期値): {}", y);
    y = 15; // 値を変更可能
    println!("可変変数 y (変更後): {}", y);

    // 型推論の例
    let inferred_string = "Hello, Rust!"; // &str と推論される
    println!("型推論された文字列: {}", inferred_string);

    // 明示的な型指定の例
    let explicit_f32: f32 = 3.14; // f32 型を明示
    println!("明示的に型指定されたf32: {}", explicit_f32);

    let explicit_u8: u8 = 255; // u8 型を明示
    println!("明示的に型指定されたu8: {}", explicit_u8);
}

fn shadowing_variables() {
    println!("\n--- 2. シャドーイング ---");

    let spaces = "   "; // spaces は文字列型 (&str)
    println!("シャドー前の spaces (文字列): '{}', 長さ: {}", spaces, spaces.len());

    let spaces = spaces.len(); // 同じ名前の新しい変数で、型は usize になる
    println!("シャドー後の spaces (数値): {}", spaces);

    let number = 10;
    println!("元の number: {}", number);

    let number = number + 5; // 値を変更しつつ、新しい変数を定義
    println!("シャドー後の number (加算): {}", number);

    {
        // 内側のスコープでシャドー
        let number = 20;
        println!("内側のスコープの number: {}", number);
    } // このスコープを抜けると内側の number は破棄される

    println!("外側のスコープの number: {}", number); // 外側の number が再び見える
}



fn constants() {
    println!("\n--- 3. 定数 (`const`) ---");

    println!("最大試行回数: {}", MAX_ATTEMPTS);
    println!("円周率: {}", PI);

    // MAX_ATTEMPTS = 10; // コンパイルエラー: 定数は変更できない
    // error[E0070]: invalid left-hand side of assignment
}

fn ownership_and_borrowing() {
    println!("\n--- 4. 所有権と借用 ---");

    // 所有権の移動 (Move)
    let s1 = String::from("Hello"); // s1がStringデータ"Hello"の所有者
    println!("s1: {}", s1);

    let s2 = s1; // s1からs2へ所有権が移動。s1はもう無効。
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // コンパイルエラー: s1は所有権を失ったため使用できない
    // error[E0382]: use of moved value: `s1`

    // データのクローン (Copy) - ヒープデータの場合、明示的にコピーする
    let s3 = String::from("World");
    let s4 = s3.clone(); // s3のデータをコピーしてs4が新しい所有者になる
    println!("s3 (コピー元): {}", s3);
    println!("s4 (コピー先): {}", s4);

    // 借用 (Borrowing)
    let mut some_string = String::from("Rust programming");

    // 不変参照 (shared borrow)
    // 複数の不変参照を同時に持つことができるが、参照先は変更できない
    let r1 = &some_string;
    let r2 = &some_string;
    println!("不変参照 r1: {}", r1);
    println!("不変参照 r2: {}", r2);
    // some_string.push_str(" is awesome!"); // エラー: 不変参照が存在するため変更できない

    // 不変参照 r1 と r2 はここで最後に使用されるため、
    // ここでそれぞれのライフタイムが終了するとコンパイラが判断します。
    // そのため、明示的な drop は不要です。

    // 可変参照 (mutable borrow)
    // 1つの可変参照のみを同時に持つことができる。参照先を変更できる
    let r3 = &mut some_string; // r1 と r2 のライフタイムが終了しているため、作成可能
    r3.push_str(" is awesome!");
    println!("可変参照 r3 (変更後): {}", r3);
    // let r4 = &mut some_string; // エラー: r3がまだ存在するため、別の可変参照は作成できない
    // let r5 = &some_string; // エラー: 可変参照が存在するため、不変参照も作成できない

    // r3がスコープを抜けると、再び変更や他の参照が可能になる
}

