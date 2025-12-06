// generic 
// to find largest  value 

// fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest{
//             largest = item ;
//         }

//     }
//     largest 


// }



// fn main (){
//     let number_list = vec![25,4,67, 90];

//     let result  = largest(&number_list);
//     println!("The largest number is :{result}");
// }



// struct Point <T> {
//     x:T,
//     y:T,

// }


// impl<T>  Point<T> {
//     fn  x (&self)-> &T{
//         &self.x

//     }

// }



// fn main (){
//     let integer = Point{x:5 , y:6};
//     let float = Point {x:3.5 , 7.8};

// }



// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }



// Traits 
// pub trait Summary {
//     fn  summarize (&self) -> String {

//     }
// }



// pub struct Newsletter {
//     pub Headline :String ,
//     pub location :String ,
//     pub author :String,
//     pub content :String ,



// }

// impl  Summary for Newsletter{
//     fn summarize(&self) -> String {
//         format!("{} , by {}  ({})", self.Headline , self.author , self.location)

//     }
// }


// Traits as Parameters

// pub fn notify (item :&Summary){
//     println!("Breaking news {}" , item.summarize())

// }


// Trait bounds  // better way to write 
// pub fn notify<T:Summary> (item :&T) {
//     println!("Breaking news {}" , item.summarize())



// }


// Clearer Trait Bounds with where Clauses

// fn some_function <T, U>(t:&T , u:&U) -> i32 
// where 
// T:Display+ clone , 
// U :Clone + Debug , {


// }



// Returning Types That Implement Traits

// fn return_summarizable () -> impl Summary {

// }



// Lifetimes 
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference

// fn main (){
//     let r ;

//     {
//         let x = 6 ;
//         r = &x ;

//     }
//     println!("the value of r is :{r}"); // will not compile r gone out of scope 
// }


// fn main (){
//     let x = 6;

//     let r = &x ;


//     println!("the value of r is :{r}");
// }


// Generic Lifetimes in Functions

// fn main (){
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest (string1.as_str() , string2);
//     println!("the longest string is :{result}"); // wil not complile due to lifetime error 
// }

// fn longest (x:&str , y:&str) ->&str{
//     if x.len()> y.len(){x} else {y}

// }


// fn main (){
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest (string1.as_str() , string2);
//     println!("the longest string is :{result}");
// }

// fn longest <'a>(x:&'a str , y:&'a str) ->&'a str{
//     if x.len()> y.len(){x} else {y}

// }


// Lifetime Annotations in Struct 
// the structs we’ve defined all hold owned types. We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
// agar struct mai ref store karna hai type mai to lifetimes use karna hoga


// struct ImportantExcerpt <'a> {
//     part : &'a str,
// }

// fn main (){
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let First_sentence = novel.split('.').next().unwrap()
//     let i = ImportantExcerpt{
//         part :First_sentence , 
//     }
// };


// Lifetime Annotations in Method Definitions

struct ImportantExcerpt <'a> {
    part : &'a str ;

}


impl <'a> ImportantExcerpt <'a> {
    fn level (&self) -> i32 {
        3

    }

}


// The Static Lifetime
//  which denotes that the affected reference can live for the entire duration of the program

let s:&'static str = String::from ("static lifetime ");
