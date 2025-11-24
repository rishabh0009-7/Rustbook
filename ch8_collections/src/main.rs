// collections are stored on the heap 
// there are 3 collections that are mostly used in rust 
//vectors , string , hashmap 


//1- vectors
//empty vector 
// let v:Vec<i32> = Vec::new();


// let v = vec![1,2,3]; // by default  type is i32 

// updating vector 
// let mut v = Vec::new();
// v.push(5);
// v.push(4);
// v.push(3);
// v.push(2);
// v.push(1);

//methods of accessing a value in a vector

// let v = vec![1,2,3,4,5];
// //method1 
// let third:&i32 = &vec[2];
// println!("The third elemst is {third}");


// //method 2 (preferred)
// let third :option<i32> = v.get(2);
// match third {
//     Some(third)=> println!("The third elemst is {third}"),
//     None => println!("There is no third element ")
// }



// // Iterating Over the Values in a Vector
// let v = vec![100, 37 , 89 ];
// for i in &v {
//     println!("{i}");
// }


// let mut  v = vec![100, 37 , 89 ];
// for i in &mut v {
//     *i+=50 ;
// }

// evctor can store value of same type but if we want to store value of  dfferent type then we can store thaat value in enum and enum  variants will be considered as same type

//  enum Spreadsheetcell{
//     Int(i32),
//     Float(f64),
//     Text(String),
// }


// let row = vec![
//     Spreadsheetcell::Int(3),
//     Spreadsheetcell::Float(8.6),
//     Spreadsheetcell::Text(String::from("black")),
// ];


// String

//empty new string 

// let mut s = String::new();


// // string with value 
// let  s = String::from("Initial contents");


// updating a string 
//push_str method 

    // let mut s = String::from("hello");
    
    // s.push_str("bar");


    // For combining strings in more complicated ways, we can instead use the format! macro

    // let s1 = String::from("tic");
    //  let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}"); // tic-tac-toe



// Indexing into Strings
// Rust strings don’t support indexing.
// The answer is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.
















































    


















