fn main() {
    let s = String::from("Rust");
    let r_s = &s; // r_s は &String 型

    // 明示的な参照外し
    println!("Length (explicit): {}", (*r_s).len());

    // 自動参照外し (推奨)
    println!("Length (implicit): {}", r_s.len()); // こちらが一般的
}