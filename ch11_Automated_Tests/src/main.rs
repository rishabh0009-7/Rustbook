// pub fn add(left :u64 , right:u64)-> u64 {
//     left + right 
// }

// #[cfg(test)]

// mod tests {
//     use super::*;
//     #[test]

// fn it_works(){
//     let result = add(2,2 );
//     assert_eq!(result , 4); // test will pass 
// }


// #[test]
// fn another (){
//     panic!("make this program fail ");
// }
// }





// #[derive(Debug)]
// struct Rectangle {
//     width:u32,
//     height:u32,
// }

// impl Rectangle  {
//     fn  can_hold (&self , other:&Rectangle )-> bool{
//         self.width> other.width && self.height > other.height

//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller (){
//         let larger = Rectangle {
//             width: 8,
//             height : 7 ,
//         };

//         let smaller = Rectangle {
//             width : 5,
//             height :1 ,
//         };

//         assert!(larger.can_hold(&smaller));


//     }
// #[test]
//     fn smaller_cannot_hold_larger (){
//         let larger = Rectangle {
//             width: 8,
//             height : 7 ,
//         };

//         let smaller = Rectangle {
//             width : 5,
//             height :1 ,
//         };

//         assert!(!smaller.can_hold(&larger));

//     }
// }


// pub fn add_two(a:u64) -> u64 {
//     a + 2
// }

// pub fn add_two(a: u64) -> u64 {
//     a + 3
// }


// #[cfg(test)]
// mod tests {
//     use super::*;


//     #[test]
//     fn it_adds_two(){
//         let result = add_two(2);
//         assert_eq!(result,4);

//     }
// }


// The assert_ne! macro will pass if the two values we give it are not equal  means expected value and actual value  are not equal and fail if they’re equal.
//Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.



// Checking for Panics with should_panic -> By adding the attribute should_panic to our test function. The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.

pub struct Guess {
    value :i32 ,
}

impl Guess {
   pub  fn  new (value:i32 )-> Guess{
        if value < 1 || value > 100{
            panic!("The value must be between 1 and 100 :{value }")
        }

        Guess{ value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic]
    fn Greater_than_100(){
        Guess::new(200);
    }
}