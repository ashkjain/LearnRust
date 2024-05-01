fn main()
{

    // Basic Referencing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Value: {}, Length: {}",s1, len);


    // Mutable Referencing
    let mut s = String::from("hello");
    println!("Unchanged: {}",s);
    change(&mut s);
    println!("Changed: {}",s);
}

// Basic Referencing
fn calculate_length(s: &String) -> usize
{
    s.len()
}
// Mutable Referencing
fn change(some_string: &mut String)
{
    some_string.push_str(", world");
}