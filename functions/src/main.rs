fn main() {
    println!("Hello, world!");

    // another_function();
    // another_function(5);
    print_labeled_measurement(5, 'h');

    // This is a statement
    let y = 6;

    // This is an expression
    let x = 5 + 5;

    // Hence why the following works
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");


    // Return value example
    let x = five();
    println!("The value of x is: {x}");
}

// Basic function calling
// fn another_function() {
//     println!("Another function.");
// }

// Basic function with parameter with its type declared
// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// Multi-parameter function
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function with return type declared
fn five() -> i32 {
    5
}
