fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants are immutable by default
    // They must have a type annotation
    // Rust's naming convention for constants is to use uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Example of shadowing
    // This example also showcases the use of an inner scope
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Floating-point numbers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Character types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuple type
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array type
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // The following is a shorthand for let a = [3, 3, 3, 3, 3];
    let a = [3; 5];


}