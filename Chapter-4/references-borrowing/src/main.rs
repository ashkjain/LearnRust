fn main() {

    /*
        // Part - 1
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.",s1, len);
    */

    /*
        // Part - 2
        let s = String::from("hello");
        change(&s);
        // This will not work since we are just referencing the variable, so we do not have ownership to modify the variable.
    */

    /*
        // Part - 3

    */
    let mut s = String::from("hello");
    println!("({}) - Original String",s);
    change(&mut s);
    println!("({}) - Updated String",s);
}

/*
    // Part - 1
    fn calculate_length(s: &String) -> usize
    {
        s.len()
    }
*/

/*
    // Part - 2
fn change(some_string: &String)
{
    some_string.push_str(", world");
}
*/

/*
    // Part - 3
*/
fn change(some_string: &mut String)
{
    some_string.push_str(", world");
}
// '&' in-front of the variable creates an immutable reference, and cannot be modified.