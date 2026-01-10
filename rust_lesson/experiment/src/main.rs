
mod utils;
mod controle;
mod structure;
mod m_enum;
mod expressions;

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

    use structure::Book;
    let mut book1 = Book::new("吾輩は猫である", "夏目漱石", 300);
    book1.borrow_book();
    book1.display_info();
    book1.return_book();
    book1.display_info();

    use m_enum::exe_enum;
    exe_enum();

    use m_enum::Point;
    let point1 = Point::new(3.0, 4.0);
    let distance = point1.distance_from_origin();
    println!("Point from origin: {}", distance);

    use m_enum::exe_generics;
    exe_generics();

    use m_enum::exec_generics;
    exec_generics();

    use m_enum::Point2;
    let p_int = Point2::new(5, 10);
    p_int.show();
}

fn example_expressions(){
    use expressions::exec_match;
    exec_match();

    use expressions::exec_match2;
    exec_match2();

    use expressions::{Coin, value_in_cents};
    let ans = value_in_cents(Coin::Penny);
    println!("gotten ans = {}", ans);
}

fn main() {
    // example_structure();
    example_expressions();
}

