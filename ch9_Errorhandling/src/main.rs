// Error handling 
//Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.

// unrecoverable error -> errors becoz of that we have to stop the program 

// fn main (){
//     panic!("crash and burn");
// }


// Recoverable error --> Most errors aren’t serious enough to require the program to stop entirely. Sometimes when a function fails it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.



// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }



// use std::fs::File;


// fn main(){
//     let greeting_file_result = File::open("hello.txt");
//     let _greeting_file = match greeting_file_result{
//         Ok(file)=> file ,
//         Err(error)=> panic!("Problem opening the file :{error:?}"),
//     };
// }

// Matching on Different Errors

use std::fs::File;
use std::io::ErrorKind;


fn main (){

    let greeting_file_result = File::open("Hello.tsxt");
    let _greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error)=> match error.kind(){
            ErrorKind::NotFound = > match file::create("hello.txt"){
                Ok(fc) => fc ,
                Err(e)=> panic!("Problem creating the file :{e:?}"),
            } , 
            _ => {
                panic!("Problem opening the file :{error:?}");
            }
        } ,
    };
}