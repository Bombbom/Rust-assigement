fn main() {
    println!("Hello, world!");
    let arr = [0, 1, 2, 4, 4, 5, 1, 2, 3];
    let sub_arr = [1, 2, 3];
    println!("{}", arr.len());
    // let distance = arr.len() - sub_arr.len() +1;
    // let sub_arr_len = sub_arr.len();
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut bool_val = false;
    while i<arr.len(){
        if arr[i] == sub_arr[j] {
            i +=1;
            j +=1;
            if j == sub_arr.len() {
                bool_val = true;
                break;
            }
        }
        else
        {
            i = i - j + 1;
            j = 0;
        }
    }
    println!("{}", bool_val);
}


// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let mut user1 = User {
//     email: String::from("someone@example.com"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
//     };
//     user1.email = String::from("anotheremail@example.com");
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

// }

// // fn build_user(email: String, username: String) -> User {
// //     User {
// //     email: email,
// //     username: username,
// //     active: true,
// //     sign_in_count: 1,
// //     }
// // }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
//     }
// fn main() {
//     let rect1 = Rectangle {
//     width: 30,
//     height: 50,
//     };
//     println!(
//     "The area of the rectangle is {} square pixels.",
//     area(&rect1)
//     );
//     println!("{:?}", rect1);
// } 
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// } 
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// } 
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//         println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// } 
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
