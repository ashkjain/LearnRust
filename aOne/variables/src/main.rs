fn main() {
    // let x = 5; // This is an immutable variable and cannot be change in the program
    let mut x: i32 = 5; // This is mutable and can be changed in the program
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
// Notes:-
/* 
Integer: 
    let int_number: i32 = 42;
    let int_unsigned: u64 = 100;
Floating-Point:
    let float_num: f32 = 3.14;
    let double_num: f64 = 6.28;
Boolean
    let bool_is: bool = true;
Character
    let char_letter: char = 'A';
String
    let str_own: String = String::from("Hello, Rust!");
    let str_slice: &str = "Hello, slice!";
 */