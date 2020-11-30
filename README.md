# __The Rust Programming Language__

Some notes collected while working through __The Rust Programming Language__.  

## Chapter 1

Anatomy of a simple program:

```rust
fn main() {
    ...
}
```

To print some text, use the `println!' macro:

```rust
println!("format string with something interpolated: {}", thing)
```

This will interpolate the variable `thing` into the format string at the position of the `{}`.

To compile:  

```bash
$rustc main.rs
```

Create a new project with cargo:  

```bash
$cargo new <project name>
```

This will create a new folder with the given project name, place the basic files and initialize git.

Build a `cargo` project:  

```bash
$cargo build  
```

Run a `cargo` project:

```bash
$cargo run  
```

## Chapter 2 - A Guessing Game

### More Language Basics

The standard library can be brought into a file with a `use` statement.  

```rust
use std::io
```

This brings in the `io` library from the `std` library.

Create variables with `let`. Mark them as mutable with `mut`.  

Static methods in Rust are invoked using the `::` syntax, e.g. `String::new()`.  

When passing parameters to a function, use `&` to pass it as a reference. References are immutable by default, so you need to mark them with `mut` if they must be manipulated.  

Functions in Rust sometimes return a `Result` type which is
an enumeration. In the case of `io::read_line()` you can call `expect()` on the returned `io::Result` instance. If all was good, it will return the number of bytes read. If there was an error, this will crash the program and print whatever message was passed to `expect()`.  

### Using Crates

You can bring additional crate libraries into your project by adding them to the dependencies secion of `Cargo.toml`.  

```toml
[dependencies]
rand = "0.5.5"
```

The next time you build, the library will be pulled down and available.

Cargo saves hard versions of dependencies into `Cargo.lock` to make sure builds can be replicated. You can use  

```bash
$cargo update
```

to update to newer versions of dependencies that are still compatible. If you want to upgrade to a version that could have breaking changes, you must edit `Cargo.toml` manually and rebuild.  

You can get at documentation for all dependencies with  

```bash
$cargo doc --open
```

Neato!  

A `match` expression takes the value of the input expression and compares it to the "arms" each of which consists of a pattern and the code to be executed if that pattern matches.  

```rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
}
```

You can "recreate" a variable of the same name which will shadow the previously declared one. This is often used when needing to convert a variable from one type to another.  

Loops are pretty much like you'd expect.

```rust
loop {
    ...
    // break; will get you out of the loop
}
```

Instead of using `expect()` on a result and just crashing, a common pattern is to use a `match` expresssion on the `Result` object returned by whatever function you are calling.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

## Chapter 3 - Common Programming Concepts  

### Variables and Mutability  

As seen before, variables are immutable by default, make them mutable with the `mut` keyword.  

```rust
let mut x: u32 = 5;
```

To create a constant, use the `const` keyword, not `let`. Constants can never be mutated.  

```rust
const BUFFER_LENGTH: u32 = 1_000_000;
```

Uppercase with underscores is convention for constant names, and `_` can be used in numeric literals. The lifetime of a constant is the entire run of the program and in the scope in which it was declared. Constants can only be set to a constant expression, not computed at runtime in any way.  

### Data Types  

#### Integers

| Length    | Signed    | Unsigned    |
| --------- |:---------:| -----------:|
| 8-bit     | `i8`      | `u8`        |
| 16-bit    | `i16`     | `u16`       |
| 32-bit    | `i32`     | `u32`       |
| 64-bit    | `i64`     | `u64`       |
| 128-bit   | `i128`    | `u128`      |
| arch      | `isize`   | `usize`     |

#### Floats

There are two floating point types in rust, `f32` and `f64`:  

```rust
let x = 2.0; // f64
let y: f32 = 2.5; // f32
```

`f64` is the default type for floats.  

#### Boolean

The `bool` type in rust has two values, `true` and `false`. Nuff' said.  

#### Char

Denoted by the `char` keyword, literals are specified with single quotes.  

```rust
let x = 'x';
```

#### Tuples  

Tuples are formed by creating a comma-separated list of values inside parentiesis. Once created, the lenght is fixed. Types can be mixed and can be inferred or explicitly declared.  

```rust
let tup: (i32, f64, char) = (25, 2.5, '2');
let (x, y, z) = tup; // x == 25, y == 2.5, z == '2'
```

Unpacking a tuple into separate variables is called destructuring. The values of a tuple can be accessed directly using `.` notation with value indices, which start at 0.  

```rust
let tup: (i32, f64, char) = (25, 2.5, '2');
println!("The first value in the tuple is {}", tup.0);
```

#### Arrays  

Arrays in rust are fixed-length once created and all elements must be of the same type. Normally, the type can be inferred, but you can explicitly type them as well.  

```rust
// basic
let a = [1,2,3,4,5];
// basic with strings
let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
// declare with type and length
let alphabet: [char; 26] = ['a', 'b', 'c', ..., 'z'];
// shorthand for filling all elements, in this case an array with 5 elements each with containing 4
let all_fours = [4; 5];
// access array elements in a familiar way, 0-based
let monday = days[1];
```

Index out of bound errors are not checked for at compile time, but will result in a runtime error rather than allow reading of invalid memory.  
