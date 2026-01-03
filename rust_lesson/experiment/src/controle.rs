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


pub fn answer_number_loop() {
    let number = get_number();
    collect_number(number);
}


pub fn count_loop() -> i32 {
    let mut count = 0;
    loop {
        count += 1;
        if count >= 13 {
            count *= 10;
            break;
        }
    }
    count
}

pub fn fizz_buzz_loop() {
    for i in 1..=50 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}


pub fn summation_for() -> f32{
    let mut sum = 0.0;
    let grades = [80, 95, 70, 100, 65];
    for grade in grades.iter() {
        sum += *grade as f32;
    }

    sum
}

pub fn find_index_for(element: String) -> i32{
    let fruits = ["apple", "banana", "cherry", "date"];
    for content in fruits.iter(){
        if *content == element {
            return fruits.iter().position(|&x| x == *content).unwrap() as i32;
        }
    }
    -1
}
