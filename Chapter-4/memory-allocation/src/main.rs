fn main() {
    let s = String::from("hello");
    println!("{}",s);

    // Variables and Data interacting with move
    let x = 5;
    let y = x;
    println!("{}",y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}",s2);
    // println!("{}",s1);

    // Variables and Data interacting with clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}",s1,s2);

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;
}
