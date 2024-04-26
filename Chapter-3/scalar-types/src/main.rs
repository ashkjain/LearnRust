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
}
