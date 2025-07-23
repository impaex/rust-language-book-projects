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

