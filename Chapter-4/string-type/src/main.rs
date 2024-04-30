fn main() {

    // The string Type

    let s = String::from("hello");
    println!("{}",s);

    let mut s = String::from("hello");
    s.push_str(", world"); // push_str adds string after the pre-defined string values of the variable
    println!("{}",s);


}
