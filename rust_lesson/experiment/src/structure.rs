// 1. 構造体の定義（設計図）
pub struct Player {
    name: String,
    hp: i32,
    level: u32,
    is_alive: bool,
}


// 2. 構造体のインスタンス化（実体の作成）
// 2. 構造体の実装（動作）
impl Player {
    // 関連関数：新しいプレイヤーを作成する（コンストラクタ的な役割）
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hp: 100,      // 初期値は100
            level: 1,     // 初期値は1
            is_alive: true,
        }
    }

    // メソッド：現在のステータスを表示する
    pub fn display_status(&self) {
        let status = if self.is_alive { "生存" } else { "死亡" };
        println!(
            "--- {} (Lv.{}) --- HP: {} [{}]",
            self.name, self.level, self.hp, status
        );
    }

    // メソッド：ダメージを受ける（自分自身を書き換えるので &mut self）
    pub fn take_damage(&mut self, damage: i32) {
        println!("{} は {} のダメージを受けた！", self.name, damage);
        self.hp -= damage;

        if self.hp <= 0 {
            self.hp = 0;
            self.is_alive = false;
            println!("{} は倒れた...", self.name);
        }
    }
}


pub struct Book {
    title: String,
    author: String,
    pages: u32,
    is_borrowed: bool
}

impl Book {
    // 2. 関連関数 new を実装してください
    pub fn new(title: &str, author: &str, pages: u32) -> Self {
        // ここに処理を書く
        Self{
            title: title.to_string(),
            author: author.to_string(),
            pages: pages,
            is_borrowed: false,
        }
    }

    // 3. メソッド borrow_book を実装してください
    pub fn borrow_book(&mut self) {
        self.is_borrowed = true;
    }

    // 4. メソッド return_book を実装してください
    pub fn return_book(&mut self) {
        self.is_borrowed = false;
    }

    pub fn display_info(&self) {
        let status = if self.is_borrowed { "貸出中" } else { "在庫あり" };
        println!(
            "\"{}\" by {} - {}ページ [{}]",
            self.title, self.author, self.pages, status
        );
    }
}
