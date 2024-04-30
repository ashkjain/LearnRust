fn main() {
    // part_one();
    // part_two();
    // part_three();
    // part_four();
    part_five();

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

/*
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
*/

/*
    fn part_three()
    {
        let mut s = String::from("hello");

        let r1 = &s; // Made a reference (Borrowed the value)
        let r2 = &s; // Made another reference (Borrowed the value)

        let r3 = &mut s; // Mutable borrow occur and caused an error. Cannot borrow as mutable cause 2 values are referencing and depend on it.

        println!("r1: {}, r2: {}, and r3: {}",r1,r2,r3);
    }
*/

/*
    fn part_four()
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("r1: {}, r2: {}",r1,r2); // Variables r1 and r2 won't be used beyond this

        let r3 = &mut s;
        println!("r3: {}",r3);
        *r3 = String::from("Changed"); // de-referenced to change tha value
        println!("{}",r3);
    }
*/

/*    fn part_five()
    {
        let reference_to_nothing = part_five_dangle();
        println!("Returned String: {}",reference_to_nothing);
    }
    // The following will not work because we cannot return just the reference, you have to pass the ownership
    /*fn part_five_dangle() -> &String
    {
        let s = String:: from("hello");

        &s
    }*/
    fn part_five_dangle() -> String
    {
        let s = String::from("hello");

        s
    }
*/