


fn main() { 

    let username = String::from("syhsamiei");
    let email = String::from("syhsamiei@gmail.com");

    let first_user = build_user(email, username); 
    println!("the user's email: {}", first_user.email); 

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User { 
        active : true,
        username: username,
        email: email,
        sign_in_count: 1, 
    }
}


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..] 
// }

// fn first_word_normal(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}




// }

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

// fn change(some_string: &mut String){
//     some_string.push_str(", World"); 
//     println!("this is my string {}", some_string); 
// }

// fn dangle() -> String { 
//     let s: String = String::from("Hello"); 
//     s
// }