//ownership
// fn main() {
//         {                      // s is not valid here, since it's not yet declared
//         let s = "hello";   // s is valid from this point forward

//         // do stuff with s
//     }                      // this scope is now over, and s is no longer valid

    

// }


// fn main (){
//     let mut s = String::from("Hello");
//     s.push_str(" world!");
//     println!("{s}");
// }



// fn main (){
//         // let s1 = String::from("hello");
//         // let s2 = s1; // now s2 is the owner of hello 
//         // println!("{s1}"); //error 


//      let mut s = String::from("hello");
//     s = String::from("ahoy");

//     println!("{s}, world!"); // this will work 






// }





// fn main (){
//     let s1 =  String::from("Hello ");
//     let s2 = s1.clone();
//     println!("s1 = {s1}, s2 = {s2}"); // this will work 


// }


// fn main (){
//     let s = String::from("hello ");
//     takes_ownership(s);
//     let x = 5 ;
//     makes_copy(x)
// }


// fn takes_ownership(some_string:String){
//     println!("{some_string}");  //hello 

// }

// fn makes_copy(some_integer:i32 ){
//     println!("{some_integer}"); //5

// }



// Referencing and borrowing 
// fn main (){
//     let s1 = String::from("hello ");
//     let (s2 , len) = calculate_length(s1);
//     println!("the length of '{s2}' is {len}.");
// }

// fn calculate_length(s:String)-> (String, usize) {
//     let length = s.len();
//     (s, length)

// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1); // address of s1 

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }




// refernces are immutable by default 
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // error 
// }



// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world"); 
// }




// if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");





    //   fn main (){
    //       let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;
    //   }



    //     let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, {r2}, and {r3}");



    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{r1} and {r2}");
    // The scopes of the immutable references r1 and r2 end after the println!where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed
    // // Variables r1 and r2 will not be used after this point.

    // let r3 = &mut s; // no problem
    // println!("{r3}");


    // string slices 

    fn main (){
        let s = String::from ("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];


        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
        // both are same 



        
    }


    














