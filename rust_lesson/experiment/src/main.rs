use rand::Rng;

mod utils;

fn main() {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..1.0);

    println!("random = {}", x);

    utils::hello_from_utils();
}

