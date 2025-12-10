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


// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary. Take a look at the following example: cargo test-- --test-threads=1

// If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with --show-output:  cargo test -- --show-output

// Running a Subset of Tests by Name ->  cargo test one_hundred 
//Only the test with the name one_hundred ran 

// Filtering to Run Multiple Tests--> cargo test add --> This command ran all tests with add in the name and filtered out the test named one_hundred.

// Ignoring Some Tests Unless Specifically Requested --> Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of cargo test. using ignore 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}



// if u want to run the ignored one --> cargo test -- --ignored 
//If you want to run all tests whether they’re ignored or not, you can run cargo test -- --include-ignored