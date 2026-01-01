// DRILL 7.1: Understanding the Problem
// First, let's see WHY lifetimes exist.

// fn main() {
//     let r;
//     {
//     let x = 5;
//         r = &x;
//     }
//     println!("{}", r)
// }

// DRILL 7.2: Fix the Dangling Reference
// Problem: Make the code compile by ensuring x lives as long as r needs it.
// fn main() {
//     let r;
//     let x = 5;
//     {
//         r = &x;
//     }
//     println!("{}", r)
// }

// DRILL 7.3: Lifetime Annotations - Basic Function
// Now let's see when YOU need to write lifetime annotations.

// Problem: This function won't compile because Rust can't figure out the lifetime of the return value:

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let s1 = String::from("long string");
//     let s2 = String::from("short");
//     let result = longest(&s1, &s2);
//     println!("{}", result);
// }

// DRILL 7.4: Lifetime Violation
// Now let's see what happens when you violate lifetime rules.
// Problem: This code SHOULD fail to compile. Try it:

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// fn main() {
//     let s1 = String::from("long string");
//     let result;
//     {
//         let s2 = String::from("short");
//         result = longest(&s1, &s2);
//     }
//     println!("{}", result)
// }

// DRILL 7.5: Multiple Lifetime Parameters
// Sometimes different references have different lifetimes.

// Problem: Write a function that takes two string references and always returns the first one.

// fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// fn main() {
//     let s1 = String::from("first");
//     let result;
//     {
//         let s2 = String::from("second");
//         result = first(&s1, &s2);
//     }
//     println!("{}", result);
// }

// DRILL 7.6: Lifetimes in Structs
// Structs can hold references, but you must annotate their lifetimes.
// Problem: Create a struct that holds a string reference:

// struct Excerpt<'a> {
//     text: &'a str,
// }

// fn main() {
//     let excerpt;
//     {
//         let novel = String::from("Call me bitch. Some years ago..");
//         let first_sentence = novel.split('.').next().unwrap();
//         excerpt = Excerpt {
//             text: first_sentence,
//         };
//     }
//     println!("{}", excerpt.text);
// }


// DRILL 7.7: Methods with Lifetimes
// Add a method to Excerpt that returns the first word:

// struct Excerpt<'a> {
//     text: &'a str,
// }

// impl <'a> Excerpt<'a> {
//     fn first_word(&self) -> &str {
//         self.text.split_whitespace().next().unwrap()
//     }
// }

// fn main() {
//     let novel = String::from("Call me bitch.");
//     let excerpt = Excerpt {text: &novel};
//     println!("{}", excerpt.first_word());
// }

// DRILL 7.8: The 'static Lifetime
// There's a special lifetime called 'static that lives for the entire program.
// Problem: Understand when references live forever:

// fn main() {
//     let s1: &'static str = "I am hardcoded in the binary";
//     let s2 = String::from("I am on the heap");

//     println!("{}", s1); 
//     // println!("{}", &s2 as &'static str);
// }

// DRILL 7.9: Combining Lifetimes and Generics
// Now let's combine what you learned: generics + lifetimes together.
// Problem: Create a struct that holds a reference and a generic value:

struct Holder<'a, T> {
    reference: &'a str,
    value: T,
}

fn main() {
    let text = String::from("hellp world");
    let holder = Holder {
        reference: &text,
        value: 42,
    };
    println!("{}, {}", holder.reference, holder.value);
}