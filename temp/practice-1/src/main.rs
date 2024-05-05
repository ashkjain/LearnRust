use std::io;

fn main()
{

    let input = user_input();
    let fahrenheit = fahrenheit_celsius(input);
    println!("The temperature in Fahrenheit is: {}, and in Celsius: {:.2}",input, fahrenheit);
}

fn user_input() -> f64
{
    println!("Please enter the temp: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<f64>()
    {
        Ok(parsed_input) => parsed_input,
        Err(_) =>
            {
                println!("Invalid Input");
                user_input()
            }
    }

}

fn fahrenheit_celsius(fahrenheit: f64) -> f64
{
    let celsius = (fahrenheit - 32.0) * (5.0/9.0);

    celsius
}

fn celsius_fahrenheit(celsius: f64) -> f64
{
    let fahrenheit = (celsius * 1.8) + 32.0;

    fahrenheit
}
