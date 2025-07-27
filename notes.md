# Data Types
### Scalar Types
Scalar types consist of the following:
- Integers
- Floating-point numbers
- Booleans
- Characters

We go over them one-by-one.

#### Integers
All integer types in Rust:
| Length   | Signed | Unsigned |
|----------|--------|----------|
| 8-bit    | i8     | u8       |
| 16-bit   | i16    | u16      |
| 32-bit   | i32    | u32      |
| 64-bit   | i64    | u64      |
| 128-bit  | i128   | u128     |
| arch     | isize  | usize    |

One can write them in decimals, hex, octal, binary and byte. 
The compiler does account for integer overflow errors in debug mode by returning an error. The compiler will not return an error in release mode, instead it will wrap the number around. 

#### Floating-point numbers
Two types, `f32` and `f64`. Default is f64 since it is (on modern CPU's) processed at roughly the same speed, but provides more precision.

#### Booleans
Written as bool, can be `true` or `false`.

#### Characters
A character value is a Unicode Scalar Value, allowing for more than just ASCII. Characters are wrapped in single quotes, while strings are wrapped in double quotes.

### Compound Types
We have:
- Tuples
- Arrays

#### Tuples
One creates a tuple by writing a comma-separated list of values inside parentheses. Tuples are of fixed length. Besides using pattern matching to destructure a tuple, one can access it's contents by using `.index` behind the variable storing the tuple, where index is the index of the value you want to access.

#### Arrays
Arrays have a single type, and are of fixed length. Arrays are useful for allocating data on the stack rather than the heap.
Accessing array elements works similarly to Python.

# Functions
Function & variable naming convention for Rust is snake case. Use the `fn` keyword.

**Statements** are instructions that perform some action and do not return a value.
**Expressions** evaluate to a resultant value.

Syntactically, only statements need a semicolon, expressions **do not**.

Return value's types must be declared in the function definition, by using an arrow. One can return early from a function by using the `return` keyword and specifying a value (as return values are not named), but most functions return the last expression implicitly.

# Comments
Works as in any other language, using `//` as comment prefix.

# Control Flow
### If Expressions
In if statements, unlike in Python and JS which automatically try to convert non-boolean types to booleans, expressions should always be a boolean type as Rust does not try to convert these.

### Loops
3 types of loops:
- `loop`
- `while`
- `for`

#### Loops
The `loop` keyword will make the code within the block execute indefinitely until explicitly asked to stop (using `break`).
Values can be returned from loops by providing a value after the `break` statement.
When having nested loops, one can provide loop labels (started with `'`) to use `break` and `continue` for the labeled loop instead of the innermost.

#### While
Conditional loops are done with while, and work similarly to other languages.

#### For
A for loop can be used to execute some code for each item in a collection.

# Ownership
Ownership follows simple rules:
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

When types with fixed lengths (which are placed on the stack) are copied or referenced, their values are copied on the stack.
```rust
let x = 5;
let y = x;
```
Now, `x` and `y` are both owners of their own value `5`, as it is copied onto the stack.

For types with variable lengths (that are placed on the heap), like `String`s, the value in the heap is not copied, only referenced twice. This would mean that the value has two owners, leading to double free errors, hence Rust makes the first owner invalid once a new variable  refers to the same value.
```rust
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!");

```
The code snippet above would error, since s1 would be dropped, hence it is not available. Errors look like:
```
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:15
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!");
  |               ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
```

In order to prevent this and make a so-called deep copy, one can use the `clone()` function on a variable that supports it. Stack only data don't have a `clone()` function, since they're not stored on the heap. Instead, they have a special annotation called the `Copy` trait. The `Copy` trait is mutually exclusive with the `Drop` trait.

### References
References allow us to use a value without transferring ownership, as this will be tedious for a lot of function calls.
References are depicted with `&` in the code:
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&s1` creates a reference that refers to the value of `s1`. `&` is used in the signature of the function to indicate that we're working with a reference instead of a variable.
Creating a reference is called **borrowing**. 
References are immutable by default, hence trying to modify one results in an error.
**Mutable references** exist:
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
Mutable references have one big restriction. If you have a mutable reference to a value, you can have no other references to that value. This restriction is in place to prevent data races at compile time. Such data races occur when:
- Two or more pointers access the same data at the same time
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

### Slices
String slices reference to only a part of the string:
```rust
    let s = String::from("hello world");

    let len = s.len();

    // hello and hello2 are equivalent
    let hello = &s[0..5];
    let hello2 = &s[..5];

    // world, world2 and world3 are equivalent
    let world = &s[6..11];
    let world2 = &s[6..len];
    let world3 = &s[6..];
``` 

This API is much harder to mess up compared to manually tracking indices of strings, as the compiler will ensure the references into the String remain valid.

String literals are of type `&str`, as they are pointing to the specific point of the binary containing the string, which is why they are immutable.

For functions working with strings, make sure to use string slices as parameters, as this creates more flexibility with working with String and str values.

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other slices
Slices exist for arrays, as well as other types of collections.

# Structs
Compared to tuples, structs name their values. Here's a struct definition:
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

Here is an instantiation of the defined struct above:
```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
As seen on the last line, dot notation is used to access values.

We're using the Owned `String` type here, because we want each struct to own all of its data. It is possible for structs to store references, but this requires the use of **lifetimes**.

When you have a parameter or variable with the same name as a value in a struct, you can make use of the Field Init Shorthand and write the following:
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Struct Update Syntax
To copy over an instance of a struct, and change only a few variables, we can do the following:
```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Since `user2` copies over `user1`'s `String` value, upon creating `user2`, `user1` is discarded as `user2` becomes the new owner of the `String` data in the heap. This also means that `user1.email`, which has not been moved over, is still accessible. 
This would not be the case if both `username` and `email` got a new value instead of being copied over, then both `user1` and `user2` would be available.

### Tuple Structs
Example:
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
Unlike normal tuples, tuple structs require you to name the type of the struct when destructuring, e.g.:
`let Point(x, y, z) = point`.

### Unit-like structs
Structs without any value. Can be useful to implement traits for types.
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```
### Method syntax
Methods are similar to functions, however methods are defined within the context of a struct (or enum or trait object), and their first parameter is always `self`. 

Methods can share a name with one of the struct fields, the only difference being the invokation, using parentheses behind the name.
`rect1.width` vs. `rect1.width()`.

#### Associated functions
Functions (not methods) can also be associated to a struct. These are typically used as constructors, as they don't need a reference to `self`. Example:
```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
These functions are invokable by using the `::` syntax, e.g.:
`let sq = Rectangle::square(3);`.

A struct can have multiple `impl` blocks, which are treated as one big one.

# Enums
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
Instances of the enum are created as follows:
```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```
The straightforward way to use these is as types in structs. However, one can also directly store data in enums, eliminating the need of a struct:
```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```
An advantage to using an enum over a struct is the fact that each variant of an enum can have different types and amounts of associated data, as shown below:
```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```
Like with structs, we can define methods on enums:
```rust
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

### Option enum
The built-in option enum serves as Rust's implementation of a null value. It's implemented as follows:
```rust
enum Option<T> {
    None,
    Some(T),
}
```
This ha been implemented so that you can safely assume any other variable that does not have the `Option` type, is a valid variable. To use the value encapsulated in the `Some(T)`, the `Option` enum provides a lot of methods to convert to the desired type.

### match control flow struct
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```
Match statements can have variables in an arm, as shown below:
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

#### Matching Option\<T> types
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

#### Matches are exhaustive
All possible types of an enum should be present in a `match` statement, else the compiler will panic.
Catch all variables can be used to make a match exhaustive, when you want code to run on only a subset of enum types. You can choose the variable name for it:
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```
One can also use `_`, which is a catch-all pattern that matches all values but doesn't bind to it, useful for when you don't need the value.
```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```
Mind that we're also using the **unit value** (the empty tuple) to denote that nothing happens on the `_` arm.

### Consise control flow with if let and else let
The `if let` control flow is a less verbose match statement used to match to one pattern and ignore the rest. The following two code snippets are equivalent:
```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
```
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```
An else statement can be added to cover the other types of the enum. The following two code snippets are equivalent:
```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
```

```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
```

Using this notation is more consise, but loses the exhaustive checking.

#### Let else
Instead of putting all the code within the `if let` block as shown below, we could use the fact that expression produce a value, as shown below the block below.
```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

The latter makes the `if let` statement easier to follow through. However, this in itself is also annoying to follow through, hence, `let else` exists. This has no if branch, only an else. When the pattern matches, it binds the value from the pattern in the outer scope, and handles the else in the `else` branch.

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

# Packages, Crates and modules
A crate is the smallest amount of code that the Rust compiler considers at the time. Two types exist, a **binary** crate or a **library** crate. The term "crate" often refers to the latter.

A package is a bundle of one or more crates that provides a set of functionality. A package must contain at least one crate, and can contain as many binary crates as desired, but only one library crate.

When creating a package using `cargo new my-project`, we find a my-project directory containing the cargo.toml and src directory. Cargo.toml does not mention the binary crate found in `src/main.rs`, as it assumes that it is the crate root of a binary crate with the same name as the package. Same holds for a library crate when finding `src/lib.rs`.

If a crate has both a `main.rs` and a `lib.rs`, it has both a binary crate and a library crate with the same name as the package.

A package can contain multiple binary crates by including files in the `src/bin` directory, where each file represents a separate binary crate.

### Modules
In the crate root file, modules can be declared using `mod garden;`. The compiler will look for the code of this module in the following places:
- Inline, within curly brackets that replace the semicolon following `mod garden`
- In the file src/garden.rs
- In the file src/garden/mod.rs

### Submodules
In any file other than the crate root, you declare submodules, e.g. `mod vegetables;`. The compiler will look for the code of this submodule in the following places:
- Inline, directly following `mod vegetables` within curly brackets instead of the semicolon
- In the file `src/garden/vegetables.rs`
- In the file `src/garden/vegetables/mod.rs`

### Paths to code in modules
Once a module is part of your crate, you can refer to code
in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.

### Private vs. public
Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. This works the same for items within a public module.

### The `use` keyword
Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

### Paths for referring to an item
To show Rust where to find an item in the module tree, we can use absolute paths, or relative paths:
- Absolute paths: starting from the crate root, for code from external crates, this is the crate name, for code from the current crate, we use literal `crate`.
- Relative paths: starting from the current module, it can use `self`, `super` (referring to a parent module) or an identifier in the current module.

Example:
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

To make these function calls compile, we would first need to make the hosting module **and** the function public, by adding the `pub` keyword to it:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The `front_of_house` module does not need a `pub` keyword, as it is not a parent module to the `eat_in_restaurant()` function, rather a sibling.

### Making structs and enums public
Structs and enums can be made public using the `pub` keyword. A struct's fields will need to be made public individually. Example:
```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```
In contrast, when making an enum public, we make all of its variants public.
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### Bringing paths into scope with the `use` keyword
As mentioned, we can use the `use` keyword to create a shortcut to avoid an otherwise long path. Note that this shortcut is only valid in the same scope of the `use` keyword. Example:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

We can provide new names for these shortcuts with the `as` keyword as follows:
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

We can also re-export names using `pub use`. This is called re-exporting because we are bringing an item into scope but also making that item available for others to bring into their scope:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

To `use` external packages, list them in the cargo.toml, and import them at the top of your scope by `use crate_name::some_item_from_it`.

One can combine `use`s from the same crate into one line by changing:
```rust
use std::cmp::Ordering;
use std::io;

use std::io;
use std::io::Write;
```
into
```rust
use std::{cmp::Ordering, io};

use std::io::{self, Write};
```

To bring all public items from a path into scope, we append the `*` glob operator: `use std::collections::*;`.


# Common collections
Unlike tuples, collections are stored on the heap, and can therefore grow and shrink in size as the program runs. Here comes a list of 3 of the most used collections.

### Vector
Allows you to store a variable number of values (of the same type) next to each other.
```rust
// Empty vector
let v: Vec<i32> = Vec::new();
// Initializing a vector with values using the vec! macro
let v = vec![1, 2, 3];

// Adding items using the push method
let mut v = Vec::new();
v.push(5);
v.push(6);

// Reading elements can be done using their index or using the get method
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}

// Iterating over immutable and mutable vectors
// Notice that the reference to the vector that the for loop holds prevents simultaneous modification of the whole vector (due to borrowing rules)
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    // * is used to dereference the value, which is required in order for the += operator to work
    *i += 50;
}

// One can use an enum to store values of different types in a vector
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### String
A string is a collection of characters.
```rust
let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

// The method also works on a literal directly, or on any type that implements the Display trait
let s = "initial contents".to_string();

// The following works too
let s = String::from("initial contents");

// Adding string slice to a String
let mut s = String::from("foo");
s.push_str("bar");

// Adding character to a String
let mut s = String::from("lo");
s.push('l');

// Combining existing Strings
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// The above compiles because the compiler can coerce the &String argument into a &str.

// Using the format! macro, we can combine Strings in more complicated ways
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```
Strings are stored in UTF-8 format in bytes. Indexing won't work as some letters take more than 1 byte, meaning one letter would take indices 0 and 1 if indexing a String was a feature in Rust.

Rust provides 3 types of representations of Strings, bytes, scalar values and grapheme clusters.

Instead of indexing, one can slice a string. This means you can select certain bytes using the slice syntax:
```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```
Iterating over strings can be done over unicode scalar values or bytes:
```rust
for c in "Зд".chars() {
    println!("{c}");
}
// Resulting in Зд

for b in "Зд".bytes() {
    println!("{b}");
}
// Resulting in 208 151 208 180 (each of the characters in the string is encoded using 2 bytes)
```

### Hash map
A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

Create a hash map as follows:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

All keys must be of the same type and all values must be of the same type.

Accessing values is done as follows:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
// copied() handles the option received from .get() by getting an Option<i32> instead of an Option<&i32>
// then unwrap_or(0) sets score to 0 if score is none or to the score
let score = scores.get(&team_name).copied().unwrap_or(0);
```
We can also loop through hash maps (note that each pair will be printed in arbitrary order):
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```
Hashmaps are owners of any owned values put in them:
```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```
When inserting references into the hash map, we need to make sure these references are valid for at least as long as the hash map is valid.

To update or overwrite a key, you insert it again:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");
```

To check whether a key has been entered into a hash map, we can use `entry()`:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{scores:?}");
```

To update a key based on it's current value, we do:
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```
The `split_whitespace` method returns an iterator over subslices, separated by whitespace, of the value in text. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here, we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk (`*`). The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.

By default, hash maps use SipHash as hashing function, which can provide resistance against DoS attacks with hash tables. If not enough, you can create your own.
