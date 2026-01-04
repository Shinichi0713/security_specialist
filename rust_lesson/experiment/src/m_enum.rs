// 1. 列挙型の定義を完成させてください
enum OrderStatus {
    Pending,
    Shipped(String),   // 追跡番号
    Cancelled(String), // 理由
}

impl OrderStatus {
    // 2. メソッドを実装してください
    pub fn print_info(&self) {
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

pub fn exe_enum() {
    let status1 = OrderStatus::Shipped(String::from("12345ABC"));
    let status2 = OrderStatus::Cancelled(String::from("在庫切れ"));
    let status3 = OrderStatus::Pending;
    
    status1.print_info();
    status2.print_info();
    status3.print_info();
}
