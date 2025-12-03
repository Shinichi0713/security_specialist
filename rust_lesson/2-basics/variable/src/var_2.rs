fn main() {
    let a = 42; // i32 型
    println!("Value of a: {}", a);
    println!("Type of a: {}", std::any::type_name::<i32>()); // i32

    let a_ref = &a; // &i32 型
    println!("\nValue of a_ref (deref): {}", *a_ref); // 参照外しで値を取得
    println!("Type of a_ref: {}", std::any::type_name::<&i32>()); // &i32

    let a_ref_ref = &a_ref; // &&i32 型
    // または &(&a) とも書ける
    println!("\nValue of a_ref_ref (deref x2): {}", **a_ref_ref); // 参照外し2回
    println!("Type of a_ref_ref: {}", std::any::type_name::<&&i32>()); // &&i32

    let a_ref_ref_ref = &a_ref_ref; // &&&i32 型
    // または &(&(&a)) とも書ける
    println!("\nValue of a_ref_ref_ref (deref x3): {}", ***a_ref_ref_ref); // 参照外し3回
    println!("Type of a_ref_ref_ref: {}", std::any::type_name::<&&&i32>()); // &&&i32

    // 各参照のアドレスを見てみる (アドレスは実行ごとに変わります)
    println!("\n--- Addresses ---");
    println!("Address of a: {:p}", &a);
    println!("Address of a_ref: {:p}", a_ref); // aが保存されているアドレス
    println!("Address of a_ref_ref: {:p}", a_ref_ref); // a_refが保存されているアドレス
    println!("Address of a_ref_ref_ref: {:p}", a_ref_ref_ref); // a_ref_refが保存されているアドレス
}


/*
Value of a: 42
Type of a: i32

Value of a_ref (deref): 42
Type of a_ref: &i32

Value of a_ref_ref (deref x2): 42
Type of a_ref_ref: &&i32

Value of a_ref_ref_ref (deref x3): 42
Type of a_ref_ref_ref: &&&i32

--- Addresses ---
Address of a: 0x4613f7f494
Address of a_ref: 0x4613f7f494
Address of a_ref_ref: 0x4613f7f548
Address of a_ref_ref_ref: 0x4613f7f600
*/
