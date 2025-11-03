// 1- variables and mutability 

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



// 2- Datatype 
 
// fn main (){

//     // integers 
//     // let x :u32 = 5;
//     // let y :i32 = -6;
//     // println!("the value of x is :{x}");
//     // println!("the value of y is :{y}");

//     // floating point 
//     // let x = 2.0; //f64 by default 
//     // let y : f32 = 5.0;
//     // println!("the value of x is :{x}");
//     // println!("the value of y is :{y}");


//     // numeric operations 
// //     fn main() {
// //     // addition
// //     let sum = 5 + 10;

// //     // subtraction
// //     let difference = 95.5 - 4.3;

// //     // multiplication
// //     let product = 4 * 30;

// //     // division
// //     let quotient = 56.7 / 32.2;
// //     let truncated = -5 / 3; // Results in -1

// //     // remainder
// //     let remainder = 43 % 5;
// // }


// // boolean 
// // let t:bool  = true ;
// // let f:bool = false ;
// // println!("The value of t is :{t}");
// // println!("The value of f is :{f}");

// // character type 
// // let c:char = 'z';

// // tuple 
// // let tup:(i32 , f64 , u8) = (500, 6.4, 5);

// // let (x,y,z) = tup ;
// // println!("the value of x is :{x}")

// // Array -> used when we know the size   and if we dont know that means size can shrink or increase we use vector 
// // let arr:[i32 , 5] = [1,2,3,4,5];
// // let first = a[0];
// //     let second = a[1];


// // let a = [3;5]; // same as a = [3,3,3,3,3];





// }



// 3 - functions 

// fn main (){
//     println!("hello ji !");

//     another_function();
// }

// fn another_function(){
//     println!("another function !");
// }


// fn main (){
//     another_function(5);
// }

// fn another_function(x:i32 ){
//     println!("The value of x is : {x}")
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


// fn main (){
//     let x = five();
//     println!("The value of x is :{x}");
// }


// fn five()-> i32 {
//     5

// }


fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // expression output is 6 
}



fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1; //statement and will give error 
}
