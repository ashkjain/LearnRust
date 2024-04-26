use std::io;
fn main() {
    
    /* 
    Integer are two type:
        starts with u = unsigned, and i = signed
        Unsigned is for zero and positive numbers
        Signed is for number from negative to positive
    All Types:
        8-bit: i8 & u8
        16-bit: i16 & u16
        32-bit: i32 & u32
        64-bit: i64 & u64
        128-bit: i128 & u128
        arch: isize & usize
    Number Literals and Examples:
        Decimal: 98_222
        Hex: 0xff
        Octal: 0o77
        Binary: 0b1111_0000
        Byte (u8 Only): b'A'
    */
    let num_one: i32 = -20;
    let num_two: u32 = 20;
    println!("Signed: {num_one}, Unsigned: {num_two}");

    /*  
    Floating Point Types
        f32: Floating 32-bit
        f64: Floating 64-bit
    */
    let x = 2.0; // f64

    let y: f32 = 3.0; //f32

    println!("Float 64: {x}, Float 32: {y}");

    /* 
    Numeric Operations

     */
    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multipication
    let product = 4 * 30;

    // divison
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Addition: {sum}, Subtraction: {difference}, Multiplication: {product}, Division: {quotient}, Truncated: {truncated}, Remainder: {remainder}");

    /*
    Boolean Type
    */
    let t = true;

    let f: bool = false; // with explicit type notation

    println!("True: {}, False: {}",t,f);

    /*
    Character Type
    */
    let c = 'z';
    let z: char = 'Z'; // With explicit type notation
    let heart_eyed_cat = '😻';

    println!("Char One: {c}, Char Two: {z}, Emoji: {}",heart_eyed_cat);

    /*
    Compound Types
    */

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup_2 = (500, 6.4, 1);
    let(x,y,z) = tup_2;

    println!("First Tuple: {}, {}, {}.\nThe Value of y is : {}, x = {x} and z = {z}",tup.0, tup.1, tup.2, y);

    let x: (i32, f64, u8) = (500,6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("Tuple Positions:-\n0: {}, 1: {}, 2: {}",five_hundred, six_point_four, one);

    /*
    Arrays
    */
    let a = [1,2,3,4,5];
    println!("Array By Positions:-\n 1(0): {}, 5(4): {}",a[0], a[4]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5]; // Fixing the length of the array

    let a = [3; 5]; // This will have value 3, 5 times in the array

    // Accessing Array elements
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];

    println!("First: {}, Second: {}",first, second);

    // Invalid Array Element Access
    let a = [1,2,3,4,5];
    println!("Please enter an array index!");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

}
