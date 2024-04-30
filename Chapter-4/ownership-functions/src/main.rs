fn main() {
    let s = String::from("hello");
    /*
    takes_ownership(s);
    check_ownership(s); // This will not work because takes_ownership() already took the value of the variable, we should consider doing clone.
    */

    takes_ownership(s.clone());
    check_ownership(s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String)
{
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32)
{
    println!("{}", some_integer)
}

fn check_ownership(some_string: String)
{
    println!("{}", some_string);
}