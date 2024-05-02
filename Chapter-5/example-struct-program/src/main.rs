#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}
fn main()
{
    // Approach 1
    let width1: u32 = 30;
    let height1: u32 = 50;
    println!("Approach 1");
    println!("The area of the rectangle is {} square pixels.",one_area(width1,height1));

    // Approach 2
    let rect1 = (30,50);
    println!("Approach 2");
    println!("The area of the rectangle is {} square pixels.",two_area(rect1));

    // Approach 3
    let rect2 = Rectangle
    {
        width: 30,
        height: 50,
    };
    println!("Approach 3");
    println!("The area of the rectangle is {} square pixels.",three_area(&rect2));

    // Approach 4
    println!("Approach 4");
    let rect3 = Rectangle
    {
        width: 30,
        height: 50,
    };
    println!("rect3 is {:?}",rect3);

    // Approach 5
    let scale = 2;
    let rect4 = Rectangle
    {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect4);
}

fn one_area(width: u32, height: u32) -> u32
{
    width * height
}

// Refactoring With Tuples
fn two_area(dimensions: (u32, u32)) -> u32
{
    dimensions.0 * dimensions.1
}

// Refactoring with structs

fn three_area(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}