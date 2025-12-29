// use flate2::Compression;
// use flate2::write::GzEncoder;
// use std::env::args;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::copy;
// use std::time::Instant;

// fn main() {
//     if args().len() != 3 {
//         eprintln!("Usage: `source` `target`");
//         return;
//     }
//     let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

//     let output = File::create(args().nth(2).unwrap()).unwrap();
//     let mut encoder = GzEncoder::new(output, Compression::default());

//     let start = Instant::now();
//     copy(&mut input, &mut encoder).unwrap();

//     let output = encoder.finish().unwrap();
//     println!(
//         "Source len: {:?}",
//         input.get_ref().metadata().unwrap().len()
//     );
//     println!("Target len:{:?}", output.metadata().unwrap().len());
//     println!("Elapsed: {:?}", start.elapsed());
// }

// RLE compression alogorithm

// use std::env;
// use std::fs;
// use std::io::Result;
// use std::time::Instant;
// fn main() -> Result<()> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 4 {
//         eprintln!(
//             "Usage: {} <compress | decompress> <input> <output>",
//             args[0]
//         );
//         std::process::exit(1);
//     }

//     let mode = &args[1];
//     let input_path = &args[2];
//     let output_path = &args[3];

//     let input_data = fs::read(input_path)?;
//     let input_size = input_data.len();
//     println!("Input size: {} bytes", input_data.len());

//     let start = Instant::now();

//     let output_data = match mode.as_str() {
//         "compress" => compress(input_data),
//         "decompress" => decompress(input_data),
//         _ => {
//             eprintln!("Invalid mode. Use 'compress' OR 'decompress'");
//             std::process::exit(1);
//         }
//     };
//     let elapsed = start.elapsed();

//     let input_bytes = input_size;  // Keep as usize for subtraction
//     let input_size_f64 = input_size as f64;
//     let output_size_f64 = output_data.len() as f64;
//     let ratio = ((output_size_f64 - input_size_f64) / input_size_f64) * 100.0;

//     println!("Output size: {} bytes", output_data.len());
//     println!("Time elapsed: {:.2?}", elapsed);

//     if ratio < 0.0 {
//         println!("Compression ratio: {:.2}% (saved {} bytes)",
//         -ratio,
//         input_bytes - output_data.len());
//     } else {
//         println!("Expansion ratio: +{:.2}% (wasted {} bytes)",
//         ratio,
//         output_data.len() - input_bytes);
//     }

//     fs::write(output_path, &output_data)?;
//     println!("Done! saved to {}", output_path);
//     Ok(())
// }

// fn compress(data: Vec<u8>) -> Vec<u8> {
//     let mut result = Vec::new();

//     if data.is_empty() {
//         return result;
//     }
//     let mut i = 0;

//     while i < data.len() {
//         let current_byte = data[i];
//         let mut count = 1;

//         while i + count < data.len() && data[i + count] == current_byte && count < 255 {
//             count += 1;
//         }
//         result.push(count as u8);
//         result.push(current_byte);
//         i += count;
//     }
//     result
// }

// fn decompress(data: Vec<u8>) -> Vec<u8> {
//     let mut result = Vec::new();

//     if data.is_empty() {
//         return result;
//     }

//     if data.len() % 2 != 0 {
//         eprintln!("Error: Invalid compressed data (odd length)");
//         return result;
//     }
//     let mut i = 0;
//     while i < data.len() {
//         let count = data[i];
//         let byte_value = data[i + 1];

//         for _ in 0..count {
//             result.push(byte_value);
//         }

//         i += 2;
//     }

//     result
// }

// Drill 1 File reader
// use std::char;
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <filename>", args[0]);
//         std::process::exit(1);
//     }

//     let filename = &args[1];
//     let data = fs::read(filename).expect("Failed to read this file");

//     println!("Total bytes: {}", data.len());

//     let text = String::from_utf8_lossy(&data);
//     let chars: Vec<char> = text.chars().take(10).collect();

//     println!("\nFirst {} characters:", chars.len());
//     for (i, char) in chars.iter().enumerate() {
//         println!("Char {}: '{}'", i, char)
//     }
// }

// Drill 2 Byte counter

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: {} <filename> <byte_value>", args[0]);
//         std::process::exit(1);
//     }
//     let filename = &args[1];
//     let byte_value: u8 = args[2]
//         .parse()
//         .expect("Please provide a valid byte (0-255)");

//     let data = fs::read(filename).expect("Failed to read this file");

//     let mut count = 0;
//     for byte in &data {
//         if *byte == byte_value {
//             count += 1;
//         }
//     }
//     println!("Bytes {} ('{}') found {} times", byte_value, byte_value as char, count);
// }

// DRILL 3 Simple filter (remove all newline characters from a file and write to new file)
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: {} <input_file> <output_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];

//     let data = fs::read(input_file).expect("Failed to read this file");
//     let mut filtered: Vec<u8> = Vec::new();

//     for byte in &data {
//         if *byte != 10 {
//             filtered.push(*byte);
//         }
//     }
//     fs::write(output_file, &filtered).expect("Failed to write file");

// }

// DRILL 4 Line Counter
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <filename>", args[0]);
//         std::process::exit(1);
//     }

//     let filename = &args[1];
//     let data = fs::read(filename).expect("Failed to read this file");

//     let mut count = 0;
//     for byte in &data {
//         if *byte == 10 {
//             count += 1;
//         }
//     }
//     println!("THe file has {} lines", count);
// }

// DRILL 5 WORD counter

// use std::env;
// use std::fs;

// fn main() {
//     let args:Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <filename>", args[0]);
//         std::process::exit(1);
//     }

//     let filename = &args[1];

//     let data = fs::read(filename).expect("Failed to read the file");

//     let mut count = 0;
//     for byte in &data {
//         if *byte == 32 {
//             count += 1;
//         }
//     }
//     if data.is_empty(){
//         println!("The file has 0 words");
//     } else {
//         println!("The file has {} words", count + 1);
//     }
// }

// FIle Copier
// use std::env;
// use std::fs;

// fn main () {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: {} <input_file> <output_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];
//     let data = fs::read(input_file).expect("Failed to read this file");

//     fs::write(output_file, &data).expect("Failed to write this file");

//     println!("successfully copied {} to {}", input_file, output_file);
// }

// DRILL 7 Uppercase Converter
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: {} <input_file> <output_file>", args[0]);
//         std::process::exit(1);
//     }
//     let input_file = &args[1];
//     let output_file = &args[2];

//     let data = fs::read(input_file).expect("Failed to read the file");
//     let mut result: Vec<u8> = Vec::new();

//     for byte in &data {
//         if *byte >= 97 && *byte <= 122 {
//             result.push(*byte - 32);
//         } else {
//             result.push(*byte);
//         }
//     }
//     fs::write(output_file, &result).expect("Failed to write the file");
//     println!("Converted file to Uppercase");
// }

// DRILL 8 Byte Replacer or Char Replacer

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 5 {
//         eprintln!("Usage: {} <input_file> <output_file> <old_bye> <new_bye>", args[0]);
//         std::process::exit(1);
//     }
//     if args[3].len() != 1 {
//         eprintln!("old_byte must be a single character");
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];

//     let old_char = args[3].chars().next().unwrap();
//     let old_byte:u8 = old_char as u8;

//     let new_char = args[4].chars().next().unwrap();
//     let new_byte:u8 = new_char as u8;

//     let data = fs::read(input_file).expect("Failed to read the file");
//     let mut result: Vec<u8> = Vec::new();
//     for byte in &data {
//         if *byte == old_byte {
//             result.push(new_byte);
//         } else {
//             result.push(*byte);
//         }
//     }
//     fs::write(output_file, &result).expect("Failed to write the file");
//     println!("Bytes replaced successfully");

// }

// Drill 9 Reverse file content
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Usage: {} <input_file> <output_file>", args[0]);
//         std::process::exit(1);
//     }
//     let input_file = &args[1];
//     let output_file = &args[2];

//     let mut data = fs::read(input_file).expect("Failed to read this file");
//     data.reverse();
//     fs::write(output_file, &data).expect("Failed to write file");
//     println!("File contents reversed successfully");
// }

// Drill 10 ROT13 cipher its letter substitution cipher

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: {} <input_file> <output_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let output_file = &args[2];

//     let data = fs::read(input_file).expect("Failed to read the file");
//     let mut result: Vec<u8> = Vec::new();

//     for byte in &data {
//         if *byte >= 97 && *byte <= 122 {
//             let position = (*byte - 97 + 13) % 26;
//             result.push(position + 97);
//         } else if *byte >= 65 && *byte <= 90 {
//             let position1 = (*byte - 65 + 13) % 26;
//             result.push(position1 + 65);
//         } else {
//             result.push(*byte);
//         }
//     }
//     fs::write(output_file, &result).expect("Failed to write this file");
//     println!("ROT13 cipher applied");
// }

// Drill 11 Frequency counter with Hashmap

// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let args:Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];

//     let data = fs::read(input_file).expect("Failed to read this file");

//     let mut map = HashMap::new();
//     for byte in &data {
//         *map.entry(*byte).or_insert(0) += 1;
//     }
//     println!("Number of unique bytes are {}", map.len());

//     for (byte, count) in map.iter() {
//         println!("Byte {} ('{}'): {} times", byte, *byte as char, count)
//     }
// }

// Drill 11 counting max bytes for byte
// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let args:Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];

//     let data = fs::read(input_file).expect("Failed to read this file");

//     let mut map = HashMap::new();
//     let mut max_bytes: Option<u8> = None;
//     let mut max_count = 0;

//     for byte in &data {
//         *map.entry(*byte).or_insert(0) += 1;
//     }

//     for (byte, count) in map.iter() {
//         if *count > max_count {
//             max_count = *count;
//             max_bytes = Some(*byte);
//         }
//     }
//     match max_bytes {
//         Some(byte) => {
//             println!("Most common bytes: {} ('{}') appears {} times", byte, byte as char, max_count);
//         } None => {
//             println!("File is empty");
//         }
//     }

// }

// Drill 13 Unique bytes with Hashset
// use std::collections::HashSet;
// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];

//     let data = fs::read(input_file).expect("Failed to read this file");
//     let mut set = HashSet::new();
//     for byte in &data {
//         set.insert(*byte);
//     }
//     println!("File contains {} unique bytes", set.len());
// }

// Drill 14 5 most common bytes

// use std::collections::HashMap;
// use std::env;
// use std::fs;

// fn main() {
//     let args:Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let data = fs::read(input_file).expect("Failed to read this file");

//     let mut map = HashMap::new();
//     for byte in &data {
//         *map.entry(*byte).or_insert(0) += 1;
//     }
//     let mut vec: Vec<(&u8, &usize)> = map.iter().collect();
//     vec.sort_by(|a, b| b.1.cmp(a.1));
//     for (i, (byte, count)) in vec.iter().take(5).enumerate() {
//         println!("{}. Byte {} ('{}'): {} times", i + 1, byte, **byte as char, count);
//     }

// }

//Drill 15 print least occured char(bytes) and their count
// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }

//     let input_file = &args[1];
//     let data = fs::read(input_file).expect("Failed to read the file");

//     let mut map = HashMap::new();
//     for byte in &data {
//         *map.entry(*byte).or_insert(0) += 1;
//     }
//     let mut vec: Vec<(&u8, &usize)> = map.iter().collect();
//     vec.sort_by(|a, b| a.1.cmp(b.1));
//     for (i, (byte, count)) in vec.iter().take(3).enumerate() {
//         println!("{}, Byte {} ('{}'): {} times", i + 1, byte, **byte as char, count);
//     }
// }

// Drill 16 Print all bytes sorted by frequency (most ot least)

// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }
//     let input_file = &args[1];
//     let data = fs::read(input_file).expect("Failed to read the file");

//     let mut map = HashMap::new();
//     for byte in &data {
//        *map.entry(*byte).or_insert(0) += 1;
//     }
//     let mut vec: Vec<(&u8, &usize)> = map.iter().collect();
//     vec.sort_by(|a, b| b.1.cmp(a.1));
//     for (i, (byte, count)) in vec.iter().enumerate() {
//         println!("{}, Byte {} ('{}'): {} times", i + 1, byte, **byte as char, count);
//     }

// }

// Drill 17 Singleton Bytes (bytes appears exactly once)

// use std::env;
// use std::fs;
// use std::collections::HashMap;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }
//     let input_file = &args[1];
//     let data = fs::read(input_file).expect("Failed to read this file");

//     let mut map = HashMap::new();
//     for byte in &data {
//         *map.entry(*byte).or_insert(0) += 1;
//     }
//     for (byte, count) in map.iter() {
//         if *count == 1 {
//             println!("Found Singleton Bytes ('{}'): {} time", *byte as char, count);
//         }
//     }
// }

// STRUCTS

// Dril 18 FileStats Struct

// use std::{collections::HashSet, env, fs};

// struct  FileStats {
//     total_bytes: usize,
//     line_count: usize,
//     word_count: usize,
//     unique_bytes: usize,
// }

// impl FileStats {
//     fn new(filename: &str) -> FileStats{
//         let data = fs::read(filename).expect("Failed to read this file");
//         let total_bytes = data.len();

//         let mut line_count: usize = 0;
//         for byte in &data {
//             if *byte == 10 {
//                 line_count += 1;
//             }
//         }

//         let mut word_count = 0;
//         for byte in &data {
//             if *byte == 32 && !data.is_empty() {
//                 word_count += 1;
//             }
//         }

//         let mut set = HashSet::new();
//         for byte in &data {
//             set.insert(*byte);
//         }
//         let unique_bytes = set.len();
//         FileStats { total_bytes, line_count, word_count, unique_bytes}
//     }
//     fn display(&self) {
//         println!("File Statistics:");
//         println!(" Total bytes: {}", self.total_bytes);
//         println!(" Word count: {}", self.word_count);
//         println!(" Line count: {}", self.line_count);
//         println!(" Unique bytes : {}", self.unique_bytes);
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: {} <input_file>", args[0]);
//         std::process::exit(1);
//     }
//     let stats = FileStats::new(&args[1]);
//     stats.display();
// }

// Drill 19 Rectangle Struct with Calculations
// use std::env;

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn new(width: u32, height: u32) -> Rectangle {
//         Rectangle { width, height }
//     }
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> u32 {
//         (self.width + self.height) * 2
//     }
//     fn display(&self) {
//         println!(" Area of rectangle is : {}", self.area());
//         println!(" Perimeter of rectangle is : {}", self.perimeter());
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Usage: {} <width>, <height>", args[0]);
//         std::process::exit(1);
//     }
//     let width: u32 = args[1].parse().expect("Width must be a number");
//     let height: u32 = args[2].parse().expect("Width must be a number");

//     let rect = Rectangle::new(width, height);
//     rect.display();
// }

// Drill 20 BookCollection Struct with Vec inside
// use std::env;

// struct BookCollection {
//     books: Vec<String>,
// }

// impl BookCollection {
//     fn new() -> BookCollection {
//         BookCollection { books: Vec::new() }
//     }

//     fn add(&mut self, title: String) {
//         self.books.push(title)
//     }
//     fn count(&self) -> usize {
//         self.books.len()
//     }
//     fn list(&self) {
//         for (i, book) in self.books.iter().enumerate() {
//             println!("{}. {}", i + 1, book);
//         }
//     }
//     fn contains(&self, title: &str) -> bool {
//         for book in &self.books {
//             if book == title {
//                 return true;
//             }
//         }
//         false
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let mut collection = BookCollection::new();

//     for i in 1..args.len() {
//         collection.add(args[i].clone());
//     }
//     println!("Books in collection: {}\n", collection.count());
//     collection.list();

//     println!(
//         "\n Searching for 'harry potter': {}",
//         if collection.contains("harry potter") {
//             "Found!"
//         } else {
//             "Not found"
//         }
//     );
// }

// Drill 21 Student Grade Calculator

// use std::env;

// struct Student {
//     name: String,
//     grades: Vec<u32>,
// }

// impl Student {
//     fn new(name:String) -> Student {
//         Student { name, grades: Vec::new() }
//     }

//     fn add_grade(&mut self, grade: u32) {
//         self.grades.push(grade);
//     }

//     fn average(&self) -> f64 {
//         if self.grades.is_empty() {
//             return 0.0;
//         }
//         let sum = self.grades.iter().sum::<u32>();
//         sum as f64 / self.grades.len() as f64
//     }

//     fn passed(&self) -> bool {
//         self.average() >= 50.0
//     }
//     fn display(&self) {
//         println!(" Student: {}", self.name);
//         println!(" Grades: {:?}", self.grades);
//         println!(" Average: {}", self.average());
//         println!(" Passed: {}✅", self.passed());
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         eprintln!("Usage: {} <name> <grade1> <grade2> <grade2> <grade3>", args[0]);
//         std::process::exit(1);
//     }
//     let mut student = Student::new(args[1].clone());
//     for i in 2..args.len() {
//         let grade: u32 = args[i].parse().expect("Grade must be a number");
//         student.add_grade(grade);
//     }
//     student.display();
// }

// Drill 22 Vec playground - Advanced Operations

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         eprintln!("Usage: {} <number1> <number2> <number3> <number4>", args[0]);
//         std::process::exit(1);
//     }

//     let mut numbers: Vec<i32> = Vec::new();
//     for i in 1..args.len() {
//         let num: i32 = args[i].parse().expect("Input must be a number");
//         numbers.push(num);
//     }

//     match numbers.get(0) {
//         Some(value) => println!("First element: {}", value),
//         None => println!("No first element"),
//     }
//     match numbers.get(100) {
//         Some(value) => println!("Value at 100: {}", value),
//         None => println!("No element at index 100 (None)"),
//     }

//     if numbers.len() > 1 {
//         println!("Before: {}", numbers[1]);
//         numbers[1] = numbers[1] * 10;
//         println!("After: {}", numbers[1]);
//     } else {
//         println!("The vector does not have at least 2 elements")
//     }

//     numbers.insert(0, 91);
//     println!("updated list: {:?}", numbers);

//     numbers.remove(0);
//     println!("Updated list again {:?}", numbers);

//     match numbers.pop() {
//         Some(value) => println!("Popped: {}", value),
//         None => println!("List is empty"),
//     }

//     println!("Original List: {:?}", numbers);
// }

// Drill 23 word counter by using advanced methof of hashmap and hashset
// use std::env;
// use std::collections::{HashMap, HashSet};

// fn main () {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         eprintln!("Usage: {} <word1> <word2>", args[0]);
//         std::process::exit(1);
//     }

//     let mut word_count: HashMap<String, u32> = HashMap::new();
//     for word in args.iter().skip(1) {
//         *word_count.entry(word.clone()).or_insert(0) += 1;
//     }
//     println!("word count {:?}", word_count);

//     println!("Looking up 'hello': {:?}", word_count.get("hello"));
//     println!("Looking up 'xyz': {:?}", word_count.get("xyz"));
//     println!("Contains key 'rust'? {}", word_count.contains_key("rust"));
//     println!("Contains key 'python'? {}", word_count.contains_key("python"));
//     println!();

//     word_count.remove("hello");
//     println!("word count {:?}",word_count);
//     for (word, count) in word_count.iter() {
//         println!("{}: {}", word , count)
//     }
//     println!();
//     let mut unique_words: HashSet<String> = HashSet::new();
//     for word in args.iter().skip(1) {
//         unique_words.insert(word.clone());
//     }
//     println!("Unique words count: {:?}", unique_words.len());
//     println!("Hashset contains 'world'? {}", unique_words.contains("world"));
//     println!("Hashset contains 'hello'? {}", unique_words.contains("hello"));
// }

// Drill 24 Traffic Light Enum with Match

// use std::{env};

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// impl TrafficLight {
//     fn action(&self) -> &str {
//         match self {
//             TrafficLight::Red => "Stop",
//             TrafficLight::Yellow => "Slow Down, Keep going",
//             TrafficLight::Green => "Go"
//         }
//     }
// }
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <red|yellow|green>", args[0]);
//         std::process::exit(1);
//     }

//     let light = match args[1].as_str() {
//         "red" => TrafficLight::Red,
//         "yellow" => TrafficLight::Yellow,
//         "green" => TrafficLight::Green,
//         _=> {
//             eprintln!("Invalid color. use red, yellow or green");
//             std::process::exit(1);
//         }
//     };
//     println!("Action: {}", light.action());
// }

// Drill 25 File Operation with Error Handling

use std::env;

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Error: cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <number1> <number2>", args[0]);
        std::process::exit(1);
    }
    let a = args[1].parse().expect("First argument must be a number");
    let b = args[2].parse().expect("Second argument must be a number");

    let result = divide(a, b);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("{}", error),
    }
}
