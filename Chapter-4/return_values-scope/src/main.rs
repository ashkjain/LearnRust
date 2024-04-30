fn main()
{
    // Part - 1
    /*
    let s1 = gives_ownership();
    println!("{}",s1);
    let s2 = String::from("hello");
    println!("{}",s2);
    let s3 = takes_and_gives_back(s2);
    println!("{}",s3);
    */

    //Part - 2
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.",s2, len);

}
// Part - 1
/*fn gives_ownership() -> String
{
    let some_string = String::from("hello - s1");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String
{
    a_string
}*/

// Part - 2
fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();
    (s,length)
}