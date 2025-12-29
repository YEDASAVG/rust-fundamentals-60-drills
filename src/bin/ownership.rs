// DRILL 4.1: Variable Declaration & Mutability
// Problem Statement:
// Understand the difference between immutable and mutable bindings.

// fn main(){
//     let x = 5;
//     // x = 6;
//     let mut y = 10;
//     y = 20;
//     println!("{}", x);
//     println!("{}", y);
// }

// DRILL 4.2: Variable Shadowing
// Problem Statement:
// Learn how shadowing differs from mutation. You can declare a new variable with the same name, even changing its type.

// fn main() {
//     let a = 14;
//     let a = a + 1;
//     let a = a * 2;
//     let a = "Abhiraj";
//     println!("{}", a); 
// }

// DRILL 4.3: Ownership - Move Semantics
// Problem Statement:
// Understand that some types (like String) move ownership when assigned or passed to functions.

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // println!("{}", s1);
//     println!("{}", s2);
// }

// DRILL 4.4: Clone - Deep Copy
// Problem Statement:
// Learn how to explicitly copy heap data when you need both variables to remain valid.

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();

//     println!("{}", s1);
//     println!("{}", s2);
// }


// DRILL 4.5: Borrowing - Immutable References
// Problem Statement:
// Learn how to borrow data without taking ownership, allowing multiple readers.


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let result = calculate_length(&s1);
//     println!("{}", result);
//     println!("{}", s1);
// }

// DRILL 4.6: Mutable References
// Problem Statement:
// Learn how to borrow data mutably to modify it without taking ownership.

// fn append_world(x: &mut String) {
//     x.push_str(" world");
// }

// fn main() {
//     let mut s1 = String::from("hello");
//     append_world(&mut s1);
//     println!("{}", s1);
// }

// DRILL 4.7: Borrowing Rules - Multiple References
// Problem Statement:
// Understand Rust's borrowing rules: you can have either multiple immutable references OR one mutable reference, but not both.

// fn main() {
//     let mut word = String::from("Hello");
//     let r1 = &word;
//     let r2 =&word;
//     // let r3 = &mut word;
    
//     println!("{}", r1);
//     println!("{}", r2);
//     // println!("{}", r3);
// }

// DRILL 4.8: Dangling References
// Problem Statement:
// Understand how Rust prevents dangling references (pointers to freed memory) at compile time.

// fn main() {
//     println!("{}", dangling());
// }

// fn dangling() -> String {
//     let word = String::from("hello");
//     word
// }

// DRILL 4.9: Slices - Borrowing Parts of Data
// Problem Statement:
// Learn about string slices - references to a portion of a String without taking ownership.

// fn main() {
//     let result = first_word("hello world");
//     println!("{}", result);
// }

// fn first_word(s: &str) -> &str {
//     if let Some(i) = s.find(' ') {
//         &s[0..i]
//     } else {
//         s
//     }
// }

// DRILL 4.10: Copy vs Move Trait
// Problem Statement:
// Understand which types implement Copy (cheap automatic copy) vs Move (transfer ownership).

fn main() {
    let x: i32 = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    let s1 = String::from("hello");
    // let s2 = s1;
    println!("{}", s1)
}