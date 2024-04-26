fn main() {
    let number = 3;
    let boolean =  true;
    if number < 5
    {
        println!("Condition was true");
    }
    else
    {
        println!("Condition was false");
    }

    if boolean
    {
        println!("This is true!");
    }
    if number != 0
    {
        println!("Number was something other than zero");
    }

    // Handling Multiple Conditions with else if

    let number = 6;

    if number % 4 == 0
    {
        println!("Number is divisible by 4");
    }
    else if number % 3 == 0
    {
        println!("Number is divisible by 3");
    }
    else if number % 2 == 0
    {
        println!("Number is divisible by 2");
    }
    else
    {
        println!("Number is not divisible by 4,3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6}; // Only first declared typed can be added in the else, otherwise error will be encountered
    // let number = if condition {5} else {"Six"}; // This will not work

    println!("The value of number is: {}",number);


    // ----------------------------
    // Repetition with loops
    // There are three types of loops in Rust: loop, while, for

    // Infinite Loop
    /*loop
    {
        println!("again!");
    }*/

    // Returning Values from loops

    let mut counter = 0;

    let result = loop
    {
        counter += 1;

        if counter == 10
        {
            break counter * 2;
        }
    };
    println!("The result is {}",result);

    // Loop labels to disambiguate between multuple loops

    let mut count = 0;
    'counting_up: loop
    {
        println!("count = {}",count);
        let mut remaining = 10;

        loop
        {
            println!("Remaining = {}",remaining);
            if remaining == 9
            {
                break;
            }
            if count == 2
            {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}",count);

    // Condition loops with while
    let mut number = 3;

    while number != 0
    {
        println!("{}!",number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // Looping through a collection with for

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5
    {
        println!("The value is: {}",a[index]);
        index += 1;
    }

    for element in a
    {
        println!("The value is: {}",element);
    }

    // For loop with reverse numerical order function
    for number in (1..4).rev()
    {
        println!("{}",number);
    }
    println!("LIFEOFF!");
}
