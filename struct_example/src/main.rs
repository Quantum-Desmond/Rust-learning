#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    // Attempt 1: just with 2 ints
    // let length1 = 50;
    // let width1 = 30;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(length1, width1)
    // );

    // Attempt 2: with tuples
    // let rect1 = (50, 30);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // Attempt 3: with structs
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // using the equivalent of static methods in C++
    let squ = Rectangle::square(3);

    println!("{:?}", squ);

}

// Attempt 1
// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// Attempt 2
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Attempt 3
// fn area(rect: &Rectangle) -> u32 {
//     rect.length * rect.width
// }