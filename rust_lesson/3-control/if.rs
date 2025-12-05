

fn main(){
    let n = 2;
    if n == 1 {
        println!("One");
    } else if n == 2 {
        println!("Two");
    } else {
        println!("Other");
    }
    let s = if n == 1 { "OK!" } else { "NG!" };
    println!({}, s);
}
