fn main(){
    let mut count = 1;
    while count != 0{
        println!("Count is: {}", count);
        count += 1;
        if count > 5 {
            count = 0;
        }
    }

    println!("Finished counting.");
}