fn main() {
    // part_one();
    part_two();
}
/*
    fn part_one()
    {
        let mut s = String::from("hello");

        let r1 = &mut s; // First Borrow here
        let r2 = &mut s; // Second Borrow here, and it will throw an error because we can make a variable that will modify the original value but not more than one.

        println!("{}, {}",r1,r2);
    }
*/
fn part_two()
{
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{} Inside the scope",r1);
        // r1 goes out of scope from here, we are good to make new reference now.
    }
    // r2 is okay to reference s now since it is different scope
    let r2 = &mut s;
    // Cannot create more reference since it is already referenced, will throw an error
    println!("{} Outside of the scope",r2);

}