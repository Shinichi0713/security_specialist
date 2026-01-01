use rand::Rng;
use std::io;

fn get_number() -> i32 {
    loop {
        println!("0から10までの数字を入力してください:");

        let mut input = String::new(); // 読み込み用のバッファ
        io::stdin()
            .read_line(&mut input)
            .expect("読み込みに失敗しました");

        match input.trim().parse() {
            Ok(num) => return num, // 正解なら値を返して関数を終了
            Err(_) => {
                println!("無効な入力です。数字を入力してください。");
                // loopなので、そのまま次の周（再試行）へ
            }
        }
    }
}

fn collect_number(mut number: i32) {
    let mut rng = rand::thread_rng();
    let target_number = rng.gen_range(0..=10); // 正解の数字を1つ決める

    loop {
        if number == target_number {
            println!("正解です！生成された数字は {} でした。", target_number);
            break; // ループを抜けて終了
        } else {
            println!("不正解です。もう一度入力してください:");
            number = get_number(); // 新しい入力を受け取ってループを続行
        }
    }
}


fn main() {
    let number = get_number();
    collect_number(number);
}