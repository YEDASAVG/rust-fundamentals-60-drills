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

// struct Holder<'a, T> {
//     reference: &'a str,
//     value: T,
// }

// fn main() {
//     let holder;
//     {
//         let text = String::from("hellp world");
//         holder = Holder {
//             reference: &text,
//             value: 42,
//         };
//     }
//     println!("{}, {}", holder.reference, holder.value);
// }

// DRILL 7.10: Lifetime Elision Rules
// Rust has 3 rules to automatically infer lifetimes. Let's learn them.

// The 3 Elision Rules:
// Each input reference gets its own lifetime
// If there's exactly ONE input lifetime, it's assigned to all output references
// If there's a &self or &mut self, its lifetime is assigned to all output references

// struct Text<'a> {
//     content: &'a str,
// }

// impl <'a> Text<'a> {
//     fn get_content(&self) -> &str {
//         self.content
//     }
// }

// fn trim(s: &str) -> &str {
//     s.trim()
// }

// fn main() {
//     let text = String::from("  hello  ");
//     let t = Text {content: &text};
//     println!("{}", t.get_content());
//     println!("{}", trim(&text));
// }

// DRILL 7.11: Lifetime Bounds on Generics
// You can require that a generic type's references live long enough.
// Problem: Create a struct that holds a generic value, but only if it can be referenced with lifetime 'a:

// use std::fmt::Display;

// struct Announcer<'a, T> {
//     message: &'a str,
//     value: &'a T,
// }

// impl<'a, T> Announcer<'a, T> where T: Display {
//     fn announce(&self) {
//         println!("{}: {}", self.message, self.value);
//     }
// }
// fn main() {
//     let num = 43;
//     let msg = "The answer is";

//     let announcer = Announcer {
//         message: msg,
//         value: &num,
//     };
//     announcer.announce();
// }

// DRILL 7.12: Return References from Methods
// Concept:
// When you return a reference from a method, Rust needs to know where that reference comes from - is it from self or from a parameter?

// struct Book<'a> {
//     title: &'a str,
//     author: &'a str,
// }

// impl <'a> Book<'a> {
//     fn get_title(&self) -> &str {
//         &self.title
//     }
//     fn get_author(&self) -> &str {
//         &self.author
//     }
//     fn get_longest(& self) -> & str {
//         if self.title.len() > self.author.len() {self.title} else {
//             self.author
//         }
//     }
// }

// fn main() {
//     let book = Book {
//         title: "The rust programming language",
//         author: "Steve Klabnik",
//     };
//     println!("Title: {}", book.get_title());
//     println!("Author: {}", book.get_author());
//     println!("Longest: {}", book.get_longest());
// }

// DRILL 7.13: Multiple References in a Struct
// Concept:
// A struct can hold multiple references with different lifetimes when they come from different sources.

// struct Context<'a, 'b> {
//     query: &'a str,
//     text: &'b str,
// }

// impl<'a, 'b> Context<'a, 'b> {
//     fn search(&self) -> bool {
//         self.text.contains(self.query)
//     }
// }

// fn main() {
//     let query = "Rust";
//     let long_text = String::from("I love Rust programming");

//     let ctx = Context {
//         query: query,
//         text: &long_text,
//     };
//     println!("Found: {}", ctx.search());
// }

// DRILL 7.14: Lifetime Subtyping
// Concept:
// When one lifetime lives longer than another, Rust can automatically use it where a shorter lifetime is expected.

// fn choose_first<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
//     first
// }

// fn main() {
//     let string1 = String::from("long string");
//     let result;
//     {
//         let string2 = String::from("short");
//         result = choose_first(&string1, &string2);
//     }
//     println!("{}", result);
// }

// DRILL 7.15: Lifetimes with Mutable References
// Concept:
// Mutable references (&mut) also need lifetime annotations, and they follow the same rules.

// fn append_suffix<'a, 'b>(text: &'a mut String, suffix: &'b str) {
//     text.push_str(suffix);
// }

// fn main() {
//     let mut message = String::from("Hello");
//     let suffix = " World";
//     append_suffix(&mut message, suffix);
//     println!("{}", message);
// }

// DRILL 7.16: Struct with Method Returning Internal Reference
// Concept:
// When a method returns a reference to internal data, the lifetime ties the returned reference to self.

// struct Container<'a> {
//     data: Vec<&'a str>
// }

// impl<'a> Container<'a> {
//     fn new() -> Container<'a> {
//         Container { data: Vec::new() }
//     }
//     fn add(&mut self, item: &'a str) {
//         self.data.push(item);
//     }
//     fn get_first(&self, index: usize) -> Option<&str> {
//         self.data.get(index).copied()
//     }
// }

// fn main() {
//     let item1 = "Rust";
//     let item2 = "Python";

//     let mut container = Container::new();
//     container.add(item1);
//     container.add(item2);

//     if let Some(first) = container.get_first(5) {
//         println!("First: {}", first);
//     } else {
//         println!("No item at that index!");
//     }

// }

// DRILL 7.17: Lifetime Annotations with Multiple Struct Fields
// Concept:
// When a struct has multiple fields with references, and you write methods that combine or compare them, you need to be careful about lifetime relationships.

// struct Pair<'a> {
//     a: &'a str,
//     b: &'a str,
// }

// impl<'a> Pair<'a> {
//     fn new(first: &'a str, second: &'a str) -> Self {
//         Self { a: first, b: second }
//     }
//     fn get_longer(&self) -> &str {
//         if self.a.len() >= self.b.len() {
//             &self.a
//         } else {
//             &self.b
//         }
//     }
//     fn concatenate(&self) -> String {
//         let mut result = String::new();
//         result.push_str(&self.a);
//         result.push_str(&self.b);
//         result
//     }
// }

// fn main() {
//     let s1 = "Hello";
//     let s2 = " World";

//     let pair = Pair::new(s1, s2);
//     println!("Longer: {}", pair.get_longer());
//     println!("Combined: {}", pair.concatenate());
// }

// DRILL 7.18: Lifetimes in Function Parameters with Structs
// Concept:
// When you pass a struct with lifetime annotations to a function, and that function returns a reference, you need to properly connect the lifetimes.

// struct Message<'a> {
//     content: &'a str
// }

// fn extract_word<'a>(msg: &Message<'a>, index: usize) -> Option<&'a str> {
//     msg.content.split_whitespace().nth(index)
// }

// fn main() {
//     let text = String::from("Rust is memory safe");
//     let msg = Message {content: &text};
//     if let Some(word) = extract_word(&msg, 3) {
//         println!("word at index 3: {}", word);
//     }
// }

// DRILL 7.19: Combining Lifetimes, Generics, and Trait Bounds
// Concept:
// Real-world code often combines all these concepts: a struct with lifetime annotations, generic types, and trait bounds, with methods that return references.

// use std::{fmt::Display};
// struct Cache<'a, T> {
//     key: &'a str,
//     value: T,
// }

// impl<'a, T> Cache<'a, T> where T: Display {
//     fn new(key: &'a str, value: T) -> Self {
//         Self { key: key, value: value }
//     }

//     fn get_key(&self) -> &str {
//         &self.key
//     }
//     fn display_value(&self) {
//         println!("Value: {}", &self.value);
//     }
// }

// fn main() {
//     let key_str = String::from("user_id");
//     let cache = Cache::new(&key_str, 15454);

//     println!("Key: {}", cache.get_key());
//     cache.display_value();
// }

// DRILL 7.20: Advanced - Lifetime Bounds with Multiple Lifetimes
// Concept:
// The final boss! A function that takes references with different lifetimes and returns a struct that properly tracks those lifetimes.

struct Comparison<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

fn compare<'a, 'b>(x: &'a str, y: &'b str) -> Comparison<'a, 'b> {
    Comparison {
        first: x,
        second: y,
    }
}

impl<'a, 'b> Comparison<'a, 'b> {
    fn get_shorter<'c>(&'c self) -> &'c str where 'a: 'c, 'b: 'c {
        if self.first.len() <= self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}
fn main() {
    let s1 = String::from("short");
    {
        let s2 = String::from("Longer String");
        let comp = compare(&s1, &s2);
        let result = comp.get_shorter();
        println!("Shorter: {}", result);
    }
}
