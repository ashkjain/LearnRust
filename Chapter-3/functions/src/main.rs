// Function
fn another_function_one()
{
    println!("Another Function.");
}

// Function with Parameters
fn another_function_two(x: i32)
{
    println!("The value of x is: {}",x);
}

fn print_labeled_measurement(value: i32, unit_label: char)
{
    println!("The measurement is: {}{}",value, unit_label);
}

// Statements and Expressions
fn expression_in_statement()
{
    let y =
        {
            let x = 3;
            x + 1
        };
    println!("The value of y is: {}",y);
}

// Function with return Values
fn five() -> i32
{
    5
}
fn plus_one(x: i32) -> i32
{
    x + 1
}
fn main() {
    println!("Hello, world!");
    another_function_one();
    another_function_two(10);
    print_labeled_measurement(5,'h');
    expression_in_statement();
    let x = five();
    println!("Value returned from function five(): {}",x);
    let y = plus_one(10);
    println!("New Value of y is: {}",y)
}