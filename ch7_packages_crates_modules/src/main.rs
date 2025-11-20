// A crate is the smallest amount of code that the Rust compiler considers at a time
// A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a command line program or a server. Each must have a function called main that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.


// Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers.


// A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.

// A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.
// Modules let us organize code within a crate for readability and easy reuse


// modules 
// use  crate::garden::vegetables::Asparagus;

// pub mod garden; // it include code in garden.rs

// fn main(){
//     let plant = Asparagus{};
//     println!("I'm growing {plant:?}");

// }



