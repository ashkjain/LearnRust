fn main()
{
    /*
        // Part - 1
        let mut s = String::from("hello");
        let word = one_first_words(&s);
    */
    /*
        // Part - 2
        // two_string_slices();
    */
        // Part - 3
        // three_string_slices();

        // Part - 4
        // four_string_slices();

        // Part - 5
        // five_string_slices();
    /*
        // Part - 6
        let mut s = String::from("hello world");
        let word = two_first_word(&s); //
        s.clear(); // Mutable borrow occurs here and throws an error since it is not mutable.
        println!("The first word is: {}",word); //
    */
    /*
        // Part - 7
        let my_string = String::from("hello world");

        let word = two_first_word(&my_string[0..6]);
        println!("Word: {}",word);
        let word = two_first_word(&my_string[..]);
        println!("Word: {}",word);

        let word = two_first_word(&my_string);
        println!("Word: {}",word);

        let my_string_literal = "hello world";

        let word = two_first_word(&my_string_literal[0..6]);
        println!("Word: {}",word);
        let word = two_first_word(&my_string_literal[..]);
        println!("Word: {}",word);

        let word = two_first_word(&my_string_literal);
        println!("Word: {}",word);
    */

    // Part - 8
    one_other_slices();
}

    fn one_first_words(s: &String) -> usize
    {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate()
        {
            println!("Position: {}, Value: {}",i,&item);
            if item == b' '
            {
                return i;
            }
        }
        s.len()
    }
    fn two_first_word(s: &str) -> &str
    {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate()
        {
            if item == b' '
            {
                return &s[0..i];
            }
        }
        &s[..]
    }

    fn one_string_slices()
    {
        let s = String::from("Hello World");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("Slice 1: {}, Slice 2: {}",hello,world);
    }

    fn two_string_slices()
    {
        let s = String::from("hello");

        let slice = &s[0..2];
        println!("Sliced: {}",slice);
        let slice = &s[..2];
        println!("Sliced: {}",slice);
    }

    fn three_string_slices()
    {
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[3..len];
        println!("Sliced: {}",slice);
        let slice = &s[3..];
        println!("Sliced: {}",slice);

    }

    fn four_string_slices()
    {
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[0..len];
        println!("Sliced: {}",slice);
        let slice = &s[..];
        println!("Sliced: {}",slice);
    }

    fn one_other_slices()
    {
        let a = [1,2,3,4,5];

        let slice = &a[1..3];
        assert_eq!(slice, &[2,3]);
        for i in slice
        {
            println!("{}",i);
        }
    }
