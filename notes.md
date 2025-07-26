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


