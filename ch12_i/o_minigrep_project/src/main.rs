// command line program 
// we’ll make our own version of the classic command line search tool grep (globally search a regular expression and print).


use std::env;

fn main (){
    let args:Vec<String> = env::args().collect(); // cl mai se arg laayega and usko collect kaedenge vector string mai 

    let query = &args[1];
    let filepath = &args[2];

    println!("Searching for query :{query}");
    println!("Searching for filepath :{filepath}");
}