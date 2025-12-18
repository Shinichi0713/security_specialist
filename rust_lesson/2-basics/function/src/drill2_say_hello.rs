macro_rules! say_hello{
    () =>{
        println!("Hello, Rustaceans!");
    };
}

fn main(){
    say_hello!();
    say_hello!();
}
