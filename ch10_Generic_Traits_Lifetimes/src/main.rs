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

