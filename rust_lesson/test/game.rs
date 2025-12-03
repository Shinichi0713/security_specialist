use rand::Rng; // ランダムな数値生成
use std::io::{self, Write}; // 標準入出力
use std::thread; // スレッドを使って少し待つ
use std::time::Duration; // 時間を定義

// プレイヤーの状態
struct Player {
    health: i32,
    gold: i32,
    inventory: Vec<String>,
}

impl Player {
    fn new() -> Self {
        Player {
            health: 100,
            gold: 0,
            inventory: vec![],
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn display_status(&self) {
        println!("\n--- プレイヤーの状態 ---");
        println!("体力: {}", self.health);
        println!("所持金: {}G", self.gold);
        println!("持ち物: {:?}", self.inventory);
        println!("------------------------");
    }
}

// イベントの種類
enum Event {
    MonsterEncounter(i32), // モンスターの強さ
    TreasureFind(i32),     // 宝物の価値
    HealPotion,            // 回復薬
    Trap,                  // 落とし穴
    Nothing,               // 何もなし
}

impl Event {
    // ランダムなイベントを生成
    fn generate_random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) { // 0から4までの乱数
            0 => Event::MonsterEncounter(rng.gen_range(10..40)),
            1 => Event::TreasureFind(rng.gen_range(20..80)),
            2 => Event::HealPotion,
            3 => Event::Trap,
            _ => Event::Nothing,
        }
    }

    // イベントの説明を返す
    fn description(&self) -> String {
        match self {
            Event::MonsterEncounter(power) => format!("凶暴なモンスターが現れた！ (強さ: {})", power),
            Event::TreasureFind(value) => format!("きらめく宝箱を見つけた！ (価値: {})", value),
            Event::HealPotion => "地面に回復薬が落ちている。".to_string(),
            Event::Trap => "目の前に怪しい仕掛けがある...".to_string(),
            Event::Nothing => "特に何も起こらなかった。".to_string(),
        }
    }
}

// メインゲームループ
fn main() {
    println!("=== 不思議な森の冒険へようこそ！ ===");
    let mut player = Player::new();
    let mut turn = 1;

    // ゲーム開始！
    while player.is_alive() && turn <= 10 { // 10ターンで終了
        println!("\n--- ターン {} ---", turn);
        player.display_status();

        let event = Event::generate_random();
        println!("{}...", event.description());

        // イベントに応じた処理をパターンマッチングで記述
        match event {
            Event::MonsterEncounter(power) => {
                println!("どうする？ [戦う/逃げる]");
                let choice = get_input();
                match choice.as_str() {
                    "戦う" => {
                        println!("モンスターに立ち向かう！");
                        thread::sleep(Duration::from_secs(1));
                        let damage = rand::thread_rng().gen_range(power / 2..power);
                        player.health -= damage;
                        println!("{}のダメージを受けた！", damage);
                        if !player.is_alive() {
                            println!("力尽きてしまった...");
                        } else {
                            println!("モンスターを倒した！");
                            let reward = rand::thread_rng().gen_range(power..power * 2);
                            player.gold += reward;
                            println!("{}Gを手に入れた！", reward);
                        }
                    }
                    _ => {
                        println!("一目散に逃げ出した！");
                        // 逃げてもダメージを受けるかも？
                        let escape_damage = rand::thread_rng().gen_range(0..power / 4);
                        if escape_damage > 0 {
                            player.health -= escape_damage;
                            println!("逃げる途中で{}のダメージを受けた...", escape_damage);
                        }
                    }
                }
            }
            Event::TreasureFind(value) => {
                println!("宝箱を開けますか？ [開ける/無視する]");
                let choice = get_input();
                if choice == "開ける" {
                    println!("宝箱を開けた！");
                    player.gold += value;
                    player.inventory.push(format!("謎の宝石 ({}G)", value)); // inventoryにアイテムを追加
                    println!("{}Gと謎の宝石を手に入れた！", value);
                } else {
                    println!("宝箱には触れず、先へ進んだ。");
                }
            }
            Event::HealPotion => {
                println!("回復薬を使いますか？ [使う/無視する]");
                let choice = get_input();
                if choice == "使う" {
                    println!("回復薬を使った！");
                    let heal_amount = rand::thread_rng().gen_range(20..50);
                    player.health += heal_amount;
                    if player.health > 100 { player.health = 100; } // 最大体力は100
                    println!("{}回復した！", heal_amount);
                } else {
                    println!("回復薬を無視した。");
                }
            }
            Event::Trap => {
                println!("罠にはまってしまった！");
                let trap_damage = rand::thread_rng().gen_range(10..30);
                player.health -= trap_damage;
                println!("{}のダメージを受けた！", trap_damage);
                if !player.is_alive() {
                    println!("力尽きてしまった...");
                }
            }
            Event::Nothing => {
                println!("平和な時間を過ごした。");
            }
        }

        turn += 1;
        thread::sleep(Duration::from_secs(2)); // 各ターンの間に少し待つ
    }

    println!("\n=== ゲーム終了！ ===");
    player.display_status();
    if player.is_alive() {
        println!("おめでとう！ {}Gと{}個のアイテムを持って生還しました！", player.gold, player.inventory.len());
    } else {
        println!("残念、冒険は志半ばで終わってしまいました...");
    }
}

// ユーザーからの入力を受け取るヘルパー関数
fn get_input() -> String {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout"); // プロンプトをすぐ表示
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string() // 余分な空白や改行を削除して返す
}