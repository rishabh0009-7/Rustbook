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



struct Point <T> {
    x:T,
    y:T,

}


impl<T>  Point<T> {
    fn  x (&self)-> &T{
        &self.x

    }

}



fn main (){
    let integer = Point{x:5 , y:6};
    let float = Point {x:3.5 , 7.8};

}



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


fn main (){
    let x = 6;

    let r = &x ;


    println!("the value of r is :{r}");
}

