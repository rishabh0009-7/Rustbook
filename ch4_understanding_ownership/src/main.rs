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


fn main (){
    let s = String::from("hello ");
    takes_ownership(s);
    let x = 5 ;
    makes_copy(x)
}


fn takes_ownership(some_string:String){
    println!("{some_string}");  //hello 

}

fn makes_copy(some_integer:i32 ){
    println!("{some_integer}"); //5

}





