// DRILL 5.1: Vec - Creating and Accessing
// Problem Statement:
// Learn the basics of Vec<T> - Rust's growable array type.

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];
//     println!("{}", numbers[2]);
//     println!("{:?}", numbers.get(10));
//     // println!("{:?}", numbers[10]);
// }


// DRILL 5.2: Vec - Push and Pop
// Problem Statement:
// Learn how to add and remove elements from a vector.

// fn main() {
//     let mut empty_vec = Vec::new();
//     empty_vec.push(10);
//     empty_vec.push(20);
//     empty_vec.push(30);

//     let popped = empty_vec.pop();

//     println!("{:?}", popped);
//     println!("{:?}", empty_vec);
// }

// DRILL 5.3: Vec - Iteration
// Problem Statement:
// Learn different ways to iterate over vectors.

// fn main() {
//     let numbers = vec![10, 20, 30, 40];

//     for i in &numbers {
//         println!("{}", i);
//     }
//     for i in numbers.iter().map(|x| x * 2) {
//         println!("{}", i);
//     }
// }

// DRILL 5.4: Vec - Mutable Iteration
// Problem Statement:
// Learn how to modify vector elements in place using mutable iteration.

// fn main(){
//     let mut numbers = vec![1, 2, 3, 4];
//     for i in numbers.iter_mut() {
//        *i *= 2;
//     }   
//     println!("{:?}", numbers);
// }


// DRILL 5.5: HashMap - Insert and Get
// Problem Statement:
// Learn Rust's hash map - a key-value store.

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<&str, i32> = HashMap::new();
//     map.insert("apple", 3);
//     map.insert("banana", 5);
//     map.insert("orange", 2);

//     let a = map.get("banana");
//     println!("{:?}", a);
//     let b = map.get("grape");
//     println!("{:?}", b);
// }

// DRILL 5.6: HashMap - Update and Entry API
// Problem Statement:
// Learn how to update values and use the entry API for conditional insertion.

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<&str, i32> = HashMap::from([
//         ("apple", 3)
//     ]);
//     map.insert("apple", 10);
//     map.entry("banana").or_insert(5);
//     map.entry("apple").or_insert(99);
//     println!("{:?}", map);
// }

// DRILL 5.7: HashMap - Iteration
// Problem Statement:
// Learn how to iterate over HashMap key-value pairs.

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<&str, i32> = HashMap::from([
//         ("apple", 3), ("banana", 5), ("orange", 2)
//     ]);
//     for (key, value) in &map {
//         println!("{}: {}", key, value);
//     }

//     for (_key, value) in map.iter_mut() {
//         *value *= 2;
//     }
//     println!("{:?}", map);
// }

// DRILL 5.8: Collecting into Collections
// Problem Statement:
// Learn how to use .collect() to transform iterators into different collection types.

// fn main() {
//     let language = vec!["rust", "java", "python"];
//     let transfrom: Vec<String> = language.iter().map(|x| x.to_uppercase()).collect();
//     println!("{:?}", transfrom);
// }

// DRILL 5.9: Filtering and Collecting
// Problem Statement:
// Combine filtering and collecting to extract specific elements

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let even_num: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
//     println!("{:?}", even_num);
// }

// DRILL 5.10: Word Frequency Counter
// Problem Statement:
// Build a practical application combining everything: count word frequency in a sentence

use std::collections::HashMap;

fn main() {

    let mut word_count = HashMap::new();
    let line = String::from("rust is great rust is fast rust is safe");
    for word in line.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count)
}