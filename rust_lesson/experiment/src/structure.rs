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