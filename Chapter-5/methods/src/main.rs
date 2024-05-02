#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}
impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn width(&self) -> bool
    {
        self.width > 0
    }

    fn square(size: u32) -> Self
    {
        Self
        {
            width: size,
            height: size,
        }
    }
}

impl Rectangle
{
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}
fn main()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle
    {
        width: 30,
        height: 50,
    };

    if rect2.width()
    {
        println!("The rectangle has a nonzero width, it is {}", rect2.width);
    }

    let rect3 = Rectangle
    {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle
    {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle
    {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}",rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}",rect3.can_hold(&rect5));

    let sq = Rectangle::square(3);
    println!("Square-Height: {}, Square-Width: {}",sq.height, sq.width);
}
