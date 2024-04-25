fn main() {
    let x: i32 = 5;
    let x: i32 = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();
    println!("This is the lenght of the spaces variabe which is an empty string: {_spaces}");
}
