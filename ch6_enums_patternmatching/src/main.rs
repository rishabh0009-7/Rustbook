// enum --> enummerate all possible variants 
// enum IpAddrKind {
//     v4, 
//     v6,
// }


// // instances 
//  let four = IpAddrKind::v4;
//  let six = IpAddrKind::v6;
// //  now both values v4 and v5 are on the same type IpAddrKind


// enum IpAddrKind {
//     v4 ,
//     v6,
// }

// struct IpAddr{
//     kind:IpAddrKind,
//     address:String,
// }

// let home = IpAddr{
//     kind:IpAddrKind::v4,
//     address:String::from("127.0.0.1")
// };


// let loopback = IpAddr{
//     kind:IpAddrKind::v6,
//     address:String::from("::1"),
// };


// just using enums  better way without structs 
// enum IpAddr{
//     v4(String),
//     v6(String),
// }


// let home = IpAddr::v4(String::from("127.0.0.1"));
// let loopback = IpAddr::v6(String::from("::1"));


// Benefits of enums over struct that we can have diff types and amount of associated data 
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("Program works!");
// // }

// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum! 
// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }



// enum message {
//     Quit,
//     Move {x:i32 , y:i32},
//     Write(String),
//     ChangeColor(i32, i32,i32),
// }


// option enum --> solves null value concept as in rust there is no null concept 

// enum option<T>{
//     None ,
//     Some(T),
// }

// fn main (){
//     let some_number = Some(6);
//     let some_char =Some('e');

//     let absent_number:option<i32> = None;

// }


// match
// enum Uststate{
//     Alabama,
//     Alaska,

// } 

// enum Coin {
//     Penny,
//     Nickel,
//     Dime, 
//     Quarter(Uststate),
// }

// fn value_in_cents(coin:Coin){
//    match coin {
//     Coin::Penny=> 1,    
//     Coin::Nickel=> 1,    
//     Coin::Dime=> 1,    
//     Coin::Quarter(state)=>{
//         println!("state quarter from {state:?}!");
//         25
//     },    
//    }

// }


//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);


// if let and let else  can be use instead of a match 

enum Uststate{
    Alabama,
    Alaska,

}
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(Uststate),
}
// let mut count = 0 ;
// match coin {
//     Coin::Quarter(state)=> println!("State quarter from {state:?}!"),
//     _=>count+=1,
// }


let mut count = 0;

if let Coin::Quarter(state) = coin{println!("State quarter from {state:?}!");}
else{
    count+=1;
}

















