
mod utils;
mod controle;
mod structure;

fn control(){
    utils::hello_from_utils();
    // controle::answer_number_loop();
    let count = controle::count_loop();
    println!("Count result: {}", count);

    controle::fizz_buzz_loop();

    let sum = controle::summation_for();
    println!("Sum of grades: {}", sum);

    let index = controle::find_index_for("cherry".to_string());
    println!("Index of target: {}", index);
}

fn example_structure(){
    use structure::Player;

    // Player構造体の新しいインスタンスを作成
    let mut player1 = Player::new("勇者");
    player1.display_status();

    // ダメージを受ける
    player1.take_damage(30);
    player1.display_status();

    player1.take_damage(80);
    player1.display_status();
}

fn main() {
    example_structure();
}

