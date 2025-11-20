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

//paths 

// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. To call a function, we need to know its path.

// A path can take two forms:

// An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
//  crate::front_of_house::hosting::add_to_waitlist();
// A relative path starts from the current module and uses self, super, or an identifier in the current module.
// front_of_house::hosting::add_to_waitlist();


