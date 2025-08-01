// New main file introducing method syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle creates an implementation block for Rectangle
// Everything in such block is associated with the Rectangle struct

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // Use of method syntax to call area on rect1
        rect1.area()
    );
}