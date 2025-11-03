// variables and mutability 

// fn main (){
//     let mut x:u32 = 5;
//     x = 6;
//     println!("The value of x is : {x}");

//     const YOU_WELL:u32 = 7  ; // in constants  we cannot use mut  and in constants naming should be uppercase  with underscore between words 
//     println!("The value of u is :{YOU_WELL}");
// }

// shadowing --> changing value while staying as an immutable variable 
// fn main (){
//     let x = 5;
//     let x = x+1 ;

//     {
//         let x = x*2 ;
//         println!("The value of x in  the inner scope is :{x}");
//     }
//     println!("The value of x  is :{x}");
// }

// using shadowing 
// fn main (){
//         let spaces = "   ";
//     let spaces = spaces.len();

// }


// using mut  we wil get error 

// fn main (){
//     let  mut spaces = "   ";
//     spaces = spaces.len();
// }



