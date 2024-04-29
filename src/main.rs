use std::{arch::aarch64::int32x4_t, io};

fn main() { 

let reference_to_nothing = dangle(); 

}

// fn gives_ownership() -> String {

//     let some_string = String::from("Hello I will learn this language in two weeks!"); 
    
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String{
//     a_string
// }

// fn calculate_length(X: i32, Y: i32) -> (String, i32, i32){
//     let sum = X + Y ; 
//     let sucs = String::from("Successful"); 
//     (sucs, X, sum)
// }


// fn calculate_length_using_reference(s: &String) -> usize{
//     s.len() 
// }

// fn change_what_we_borrow(s: &String) { 
//     // we cannot change what we borrowed !! instead of this function I will declare another function using mutable references!!
//     s.push_str(", world"); 
// }

fn change(some_string: &mut String){
    some_string.push_str(", World"); 
    println!("this is my string {}", some_string); 
}

fn dangle() -> String { 
    let s: String = String::from("Hello"); 
    s
}