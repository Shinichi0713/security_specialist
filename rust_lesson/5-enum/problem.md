
### 実践コーディング課題：配送管理システム

クイズの復習として、実際に以下のコードをエディタ（VSCode等）に貼り付けて、指示に従って完成させてみてください。

```rust
// 1. 列挙型の定義を完成させてください
enum OrderStatus {
    Pending,
    Shipped(String),   // 追跡番号
    Cancelled(String), // 理由
}

impl OrderStatus {
    // 2. メソッドを実装してください
    fn print_info(&self) {
        // ここに match 式を書いて、状態ごとに異なるメッセージを出力してください
        // Pending -> "準備中です"
        // Shipped -> "発送済み。追跡番号: [番号]"
        // Cancelled -> "キャンセルされました。理由: [理由]"
        match self {
            OrderStatus::Pending => println!("準備中です"),
            OrderStatus::Shipped(id) => println!("発送済み。追跡番号: {}", id),
            OrderStatus::Cancelled(reason) => println!("キャンセルされました。理由: {}", reason),
        }
    }
}

fn main() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Shipped(String::from("ABC-12345"));
    
    status1.print_info();
    status2.print_info();
}

```

構造体（struct）が「情報の静的な塊」であるのに対し、列挙型（enum）は「動的な状態の変化」を表現するのに最適です。ぜひ実際に手を動かして、`match` 式の便利さを体感してみてください！応援しています。