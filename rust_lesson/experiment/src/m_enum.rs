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


// 1. ジェネリックな構造体 Point を定義してください
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 2. new メソッドを実装してください
    pub fn new(x: T, y: T) -> Self {
        // ここに処理を書く
        Point { x, y }
    }

    // 3. x と y を入れ替えた新しい Point を返すメソッドを実装してください
    pub fn swap_xy(self) -> Point<T> {
        // ここに処理を書く
        Point { x: self.y, y: self.x }
    }
}

// 4. f64 型の時だけ使えるメソッドを定義（具体的な型を指定する impl）
impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        // 原点からの距離 = sqrt(x^2 + y^2)
        // ヒント: self.x.powi(2) や self.x.sqrt() を使います
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn exe_generics() {
    // 整数型の Point
    let p1 = Point::new(10, 20);
    let p1_swapped = p1.swap_xy();
    println!("p1 swapped: x={}, y={}", p1_swapped.x, p1_swapped.y);

    // 浮動小数点型の Point
    let p2 = Point::new(3.0, 4.0);
    println!("p2 distance: {}", p2.distance_from_origin());
}
