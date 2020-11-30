# __The Rust Programming Language__

Some notes collected while working through __The Rust Programming Language__.  

## Chapter 1

Anatomy of a simple program:

    fn main() {
        ...
    }

To print some text, use the `println!' macro:

    println!("format string with something interpolated: {}", thing)

This will interpolate the variable `thing` into the format string at the position of the `{}`.

To compile:  

    $ rustc main.rs

Create a new project with cargo:  

    $ cargo new <project name>

This will create a new folder with the given project name, place the basic files and initialize git.

Build a `cargo` project:  

    $ cargo build  

Run a `cargo` project:

    $ cargo run  


## Chapter 2 - A Guessing Game

### More Language Basics

The standard library can be brought into a file with a `use` statement.  

    use std::io

This brings in the `io` library from the `std` library.

Create variables with `let`. Mark them as mutable with `mut`.  

Static methods in Rust are invoked using the `::` syntax, e.g. `String::new()`.  

When passing parameters to a function, use `&` to pass it as a reference. References are immutable by default, so you need to mark them with `mut` if they must be manipulated.  

Functions in Rust sometimes return a `Result` type which is
an enumeration. In the case of `io::read_line()` you can call `expect()` on the returned `io::Result` instance. If all was good, it will return the number of bytes read. If there was an error, this will crash the program and print whatever message was passed to `expect()`.  

### Using Crates

You can bring additional crate libraries into your project by adding them to the dependencies secion of `Cargo.toml`.  

    [dependencies]
    rand = "0.5.5"

The next time you build, the library will be pulled down and available.

Cargo saves hard versions of dependencies into `Cargo.lock` to make sure builds can be replicated. you can use 

    $ cargo update

to update to newer versions of dependencies that are still compatible. If you want to upgrade to a version that could have breaking changes, you must edit `Cargo.toml` manually and rebuild.  

You can get at documentation for all dependencies with 

    $ cargo doc --open

Neato!  

A `match` expression takes the value of the input expression and compares it to the "arms" each of which consists of a pattern and the code to be executed if that pattern matches.  

You can "recreate" a variable of the same name which will shadow the previously declared one. This is often used when needing to convert a variable from one type to another.  









