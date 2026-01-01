// DRILL 6.1: Basic Generic Struct
// Problem Statement:
// Create a generic struct that can hold any single value and implement a method to retrieve it.

// struct Wrapper<T>{
//     value: T
// }

// impl<T> Wrapper<T> {
//     fn new(value: T) -> Wrapper<T> {
//         Wrapper { value }
//     }
//     fn get(&self) -> &T {
//         &self.value
//     }
// }

// fn main(){
//     let num_wrapper = Wrapper::new(55);
//     println!("{}", num_wrapper.get());

//     let str_wrapper = Wrapper::new("Hello");
//     println!("{}", str_wrapper.get());
// }

// DRILL 6.2: Generic Struct with Two Types
// Problem Statement:
// Create a generic struct that can hold two values of different types.
// struct Point<T, U> {
//     first: T,
//     second: U,
// }

// impl <T, U> Point<T, U> {
//     fn new(first: T, second: U) -> Point<T, U> {
//         Point { first, second }
//     }
//     fn get_first(&self) -> &T {
//         &self.first
//     }
//     fn get_second(&self) -> &U {
//         &self.second
//     }
// }
// fn main() {
//     let num = Point::new(10, "Hello");
//     println!("{}", num.get_first());
//     println!("{}", num.get_second());
// }

// DRILL 6.3: Generic Method with Different Return Type
// Problem Statement:
// Learn how methods can introduce their own generic types separate from the struct's generics.
// #[derive(Debug)]
// struct Point<T, U> {
//     first: T,
//     second: U,
// }
// impl<T:Copy, U> Point<T, U> {
//     fn mix<V>(&self, other: V) -> Point<T, V> {
//         Point { first: self.first, second: other }
//     }
// }

// fn main() {
//     let p1: Point<i32, String> =Point {first: 3, second: String::from("hello")};
//     let p2: Point<i32, bool> = p1.mix(true);
//     println!("{}", p2.first);
//     println!("{}", p2.second);
// }

// DRILL 6.4: Generic Trait Bounds - Display
// Problem Statement:
// Learn how to constrain generics so they can be printed with println!

// use std::fmt::Display;

// struct Container<T> {
//     value: T
// }

// impl<T: Display> Container<T> {
//     fn print(&self) {
//         println!("{}", self.value)
//     }
// }

// fn main() {
//     let num_container = Container {value: 42};
//     num_container.print();

//     let word_container = Container {value: String::from("Hello Rust!")};
//     word_container.print();
// }

// DRILL 6.5: Multiple Trait Bounds
// Problem Statement:
// Learn how to require multiple traits on a generic type using the + operator.

// use std::fmt::Display;

// struct Comparator<T> {
//     value: T
// }

// impl <T: Display + PartialEq> Comparator<T> {
//     fn compare_and_print(&self, other: T) {
//         let result = self.value == other;
//         println!("Values are equal: {}", result);
//     }
// }

// fn main() {
//     let first = Comparator {value: 42};
//     first.compare_and_print(42);

//     let second = Comparator {value: 42};
//     second.compare_and_print(10);
// }

// DRILL 6.6: Where Clauses
// Problem Statement:
// Learn the where clause syntax for cleaner code when you have complex trait bounds.

// use std::fmt::Display;

// struct Processor<T, U> {
//     input: T,
//     output: U,
// }

// impl<T, U> Processor<T, U>
// where
//     T: Display,
//     U: Display,
// {
//     fn process(&self) {
//         println!("Input: {}, Output: {}", self.input, self.output);
//     }
// }

// fn main() {
//     let result: Processor<i32, String> = Processor {
//         input: 42,
//         output: String::from("Hello"),
//     };
//     result.process();
// }

// DRILL 6.7: Generic Enums
// Problem Statement:
// Learn that enums can be generic too (like Option<T> and Result<T, E>).

// enum Operation<T> {
//     Add(T, T),
//     Subtract(T, T),
//     None,
// }

// fn execute<T>(op: Operation<T>) -> Option<T>
// where
//     T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
// {
//     match op {
//         Operation::Add(a, b) => Some(a + b),
//         Operation::Subtract(a, b) => Some(a - b),
//         Operation::None => None,
//     }
// }

// fn main() {
//     let result1 = execute(Operation::Add(5, 2));
//     println!("{:?}", result1);
//     let result2 = execute(Operation::Subtract(5, 2));
//     println!("{:?}", result2);
// }


// DRILL 6.8: Generic Functions - Find Max
// Problem Statement:
// Learn to write the most common pattern: generic functions that work with any type meeting constraints

// fn find_max<T>(items: &[T]) -> Option<&T> where T: PartialOrd {
//     if items.is_empty() {
//         return None;
//     }
//     let mut max = &items[0];

//     for item in items.iter() {
//         if item > max {
//             max = item;
//         }
//     }
//     Some(max)
// }

// fn main() {
//     let num = vec![3, 7, 2, 9, 1];
//     let result = find_max(&num);
//     println!("{:?}", result);

//     let words = vec!["rust", "go", "python"];
//     let result2 = find_max(&words);
//     println!("{:?}", result2);
// }

// DRILL 6.9: Combining Generics with Ownership
// Problem Statement:
// Learn how to write a generic function that takes ownership and returns owned values.

// fn wrap_in_vec<T>(item: T) -> Vec<T> {
//     vec![item]
// }

// fn main() {
//     let num = 42;
//     let word = String::from("Rust");

//     println!("{:?}", wrap_in_vec(num));
//     println!("{:?}", wrap_in_vec(word));
// }

// DRILL 6.10: Real-World Pattern - Generic Cache
// Problem Statement:
// Build a practical generic cache structure that stores key-value pairs.

// DRILL 6.10: Real-World Pattern - Generic Cache
// Problem Statement:
// Build a practical generic cache structure that stores key-value pairs.

// struct Cache<K, V> {
//     data: Vec<(K, V)>
// }

// impl<K, V> Cache<K, V> {
//     fn new() -> Cache<K, V> {
//         Cache { data: Vec::new() }
//     }
//     fn insert(&mut self, key: K, value: V) {
//         self.data.push((key, value));
//     }

//     fn get(&self, key: &K) -> Option<&V> where K:PartialEq {
//          self.data.iter().find(|(k, _v)| k == key). map(|(_k, v)| v) 
//     }
// }

// fn main() {
//     let mut cache = Cache::new();
//     cache.insert(String::from("Score"), 100);
//     cache.insert(String::from("Level"), 5);

//     let result = cache.get(&String::from("level"));
//     println!("{:?}", result);
// }

// Create a generic struct Box<T> that holds one value
// Add a method swap() that takes another Box<T> and swaps their values
// Test it with integers and strings

struct Box<T> {
    value: T
}

impl<T> Box<T> {
    fn new(value: T) -> Box<T> {
        Box { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn swap(&mut self, other: &mut Box<T>) {
        std::mem::swap(&mut self.value, &mut other.value);
    }
}

fn main() {
    let mut box1 = Box::new(10);
    let mut box2 = Box::new(20);
    box1.swap(&mut box2);
    println!("{}, {}", box1.get(), box2.get()); // Should print: 20, 10
}