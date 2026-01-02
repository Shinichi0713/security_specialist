
mod utils;
mod controle;

fn control(){
    utils::hello_from_utils();
    // controle::answer_number_loop();
    let count = controle::count_loop();
    println!("Count result: {}", count);

    controle::fizz_buzz_loop();
}


fn main() {
    control();
}

