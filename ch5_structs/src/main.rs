// // structs


// struct User {
//     active :bool,
//     username:String
//     email:String,
//     sign_in_account:u64,

// }

// fn main (){
//     let user1 = User{
//         active:True,
//         Username:String::from("Rizz"),
//         email:String::from("rizzlifts2024@gmail.com"),
//         sign_in_account:1,
//     };

//     user1.email = String::from("rizzcodes@gmail.com");


   
   

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };

//     let user2 = User {
//            email: String::from("another2@example.com"),
//            ..user1 // this means same as user1 
    

//     };
// }




// write a program that calculates the area of a rectangle using structs 

// struct Rectangle{
//     width:u32,
//     height:u32,

// }

// fn main (){
//     let rec1 = Rectangle{
//         width:20,
//         height:30,

//     };

//     println!("the area of rectangle is :{}" , area(&rec1));
// }

// fn area (rectangle:&Rectangle)->u32{
//     rectangle.width*rectangle.height

// }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}"); //  to print rect1 we have to use debug trait 
// }



#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}


impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height

    }
}


fn main (){
    let rec1 = Rectangle{
        width :20,
        height:30,

    };
    println!("the area of rectangle is :{}", rec1.area());
}
