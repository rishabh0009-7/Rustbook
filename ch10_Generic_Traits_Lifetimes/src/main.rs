// generic 
// to find largest  value 

fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = item ;
        }

    }
    largest 


}



fn main (){
    let number_list = vec![25,4,67, 90];

    let result  = largest(&number_list);
    println!("The largest number is :{result}");
}
