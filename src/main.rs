
fn main() { 

    let username = String::from("syhsamiei");
    let email = String::from("syhsamiei@gmail.com");

    let first_user = build_user_first_approach(&email, &username); 
    println!("the user's email: {}", first_user.email); 

    let second_user = build_user_shorthand(email, username); 
    println!("the new user's username: {}", second_user.username); 

    let third_user = User{ 
        active : first_user.active,
        username : first_user.username,
        email: String::from("another@gmail.com"),
        sign_in_count : first_user.sign_in_count,
    }; 
    println!("the third user's email: {:#?}", third_user); 

    let forth_user = User{
        email: String::from("shsamiei27ap@gmail.com"),
        ..third_user
    };

    println!("the forth user's username: {}", forth_user.email); 

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); 


    println!("the first item is : {}", black.1); 



    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    let sq = Rectangle::square(3); 
    println!("the shape is : {:?}", sq); 
}

// lets make a struct with methods : 

#[derive(Debug)]
struct Rectangle { 
    width : u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self{
        Self {
            width : size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// lets create some tuple struct 

struct Color(i32, i32, i32); 
struct Point(i32, i32, i32); 

// unit like struct without any fields
struct AlwaysEqual;

fn build_user_first_approach(email: &String, username: &String) -> User {
    let user = username.clone(); 
    let email = email.clone();
    User { 
        active : true,
        username: user,
        email: email,
        sign_in_count: 1, 
    }
}

fn build_user_shorthand(email: String, username: String) -> User { 
    User{
        active : true,
        username,
        email,
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