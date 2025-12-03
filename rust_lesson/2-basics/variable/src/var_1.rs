fn main() {
    let a = 42;
    let a_ref_ref_ref = &&& a;
    println!("The value of a is: {}", a_ref_ref_ref);

    let a_ref = **a_ref_ref_ref;
    let a_value = *a_ref;
    println!("The value of a_ref is: {}", a_ref);
    println!("The value of a_value is: {}", a_value);
}