#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementation block
    fn area(&self) -> u32 { // &self instead of rectangle: &Rectangle because rust knows self is Rectangle
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        // area(width1, height1)
        // area(rect1)
        // area(&rect1)
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::new(10);
}

// fn area(width: u32, height: u32) -> u32 {
// fn area(dimensions: (u32, u32)) -> u32 {
// fn area(rectangle: &Rectangle) -> u32 {
    // width * height
    // dimensions.0 * dimensions.1
    // rectangle.width * rectangle.height
// }
