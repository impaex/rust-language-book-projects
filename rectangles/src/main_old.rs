// Initial version
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Version with tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Version with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // Since rectangle doesn't implement the Display trait, we can't print it directly as it would result in an error
    // println!("rect1 is {}", rect1);
    // Instead, we use :?, which tells Rust to use the Debug trait for printing
    // This however does not work unless we specify that Rectangle implements the Debug trait, as shown in line 31
    println!("rect1 is {rect1:?}");
    // Using pretty print (:#?)
    println!("rect1 is {rect1:#?}");

    // One can also use the dbg! macro to print debug information
    // dbg! is a macro that takes ownership of an expression, 
    // prints the file and line number, along with the value of that expression, and
    // returns ownership of the expression
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}