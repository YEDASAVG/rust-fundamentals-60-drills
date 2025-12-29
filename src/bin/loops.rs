// DRILL 1.1: Count to Freedom
// Problem: Print numbers from 1 to 10, each on a new line
// Use a for loop, output should include 10

// fn main() {
//     for i in 1..=10 {
//         println!("{}", i);
//     }
// }

// DRILL 1.2: Reverse Countdown
// Problem: Create a countdown timer that prints from 10 down to 1, then prints "Liftoff!"
// Use a for loop, must count DOWN

// fn main() {
//     for i in (1..=10).rev() {
//         println!("{}", i);
//     }
//     println!("Liftoff!");
// }

// DRILL 1.3: Step Jumper
// Problem: Print only even numbers from 0 to 20
// Use a for loop, don't use if statement to filter, make loop skip numbers

// fn main() {
//     for i in (0..=20).step_by(2) {
//         println!("{}", i);
//     }
// }

// DRILL 1.4: Vector Walker
// Problem: Print fruit names with their position number (0: apple, 1: banana, etc.)
// Use a for loop to iterate over vector, show both index and value

// fn main() {
//     let fruits = ["apple", "banana", "cherry", "date"];
//     for (i, count) in fruits.iter().enumerate() {
//         println!("{}: {}", i, count);
//     }
// }

// DRILL 1.5: String Character Explorer
// Problem: Print each character of string "Rust" with its byte position (0: R, 1: u, etc.)
// Iterate over string's characters, show both position and character

// fn main() {
//     let text = "Rust";
//     for (i, count) in text.chars().enumerate() {
//         println!("{}: {}", i, count);
//     }
// }

// DRILL 1.6: While Counter
// Problem: Print numbers from 1 to 5 using a while loop (not for loop)
// Use a counter variable, manually manage increment

// fn main() {
//     let mut count = 0;
//     while count < 5 {
//         count += 1;
//         println!("{}", count);
//     }
// }

// DRILL 1.7: Conditional While Loop
// Problem: Keep doubling a number starting from 1 until it exceeds 100, print each value (1, 2, 4, 8...128)
// Use while loop, condition checks the value itself not a counter

// fn main() {
//     let mut increment = 1;
//     while increment <= 100 {
//         println!("{}", increment);
//         increment = increment * 2;
//     }
// }

// DRILL 1.8: Infinite Loop with Break
// Problem: Use loop keyword (infinite loop) to print 1 to 3, then exit using break
// Must use loop (not while), exit with break, use counter variable

// fn main() {
//     let mut i = 1;
//     loop {
//         if i <= 3 {
//             println!("{}", i);
//             i += 1;
//         } else {
//             break;
//         }
//     }
// }

// DRILL 1.9: Continue Keyword
// Problem: Print numbers 1 to 10 but skip number 5 (output: 1,2,3,4,6,7,8,9,10)
// Use continue keyword (not just if wrapping print), loop visits all numbers

// fn main() {
//     for i in 1..=10 {
//         if i == 5 {
//             continue;
//         }
//         println!("{}", i);
//     }
// }

// DRILL 1.10: Nested Loops - Multiplication Table
// Problem: Print 3x3 multiplication table (1 2 3 / 2 4 6 / 3 6 9)
// Use nested loops (loop inside loop), outer=rows, inner=columns, print row*col

// fn main() {
//     for row in 1..=3 {
//         for col in 1..=3 {
//             let x = row * col;
//             print!("{} ", x);
//         }
//         println!();
//     }
// }

// DRILL 1.11: Loop Labels
// Problem Statement:
// Create two nested loops. Break out of BOTH loops when a condition is met (not just the inner one)
// fn main() {
//     'outer: for row in 1..=3 {
//         for col in 1..=3 {
//             println!("Row {} Col {}", row, col);
//             if row == 2 && col == 2 {
//                 break 'outer;
//             }
//         }
//     }
// }

// DRILL 1.12: Iterator Chain - Filter and Map
// Problem Statement:
// You have a vector of numbers 1 to 10. Print only the even numbers, but doubled.

// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     for num in numbers.iter().filter(|num| *num % 2 == 0).map(|num| num * 2) {
//         println!("{}", num);
//     }
// }


// DRILL 1.13: Collect into Vector
// Problem Statement:
// Take a range 1 to 5, square each number, and store the results in a new vector. Then print the vector.

// fn main() {
//     let mut squares: Vec<i32> = (1..=5).map(|x| x * x ).collect();
//     println!("{:?}", squares);

//     for num in &mut squares.iter().map(|num| num % 2 == 0) {
//         println!("{}", num);
//     }
// }

//  DRILL 1.14: Sum with Iterator
// Problem Statement:
// Calculate the sum of all numbers from 1 to 100 using an iterator method (not a loop).

// fn main() {
//     let sum: i32 = (1..=100).sum();
//     println!("Sum: {}", sum);
// }

// DRILL 1.15: Find First Match
// Problem Statement:
// Find the first number greater than 50 in a vector, using an iterator method.
// fn main() {
//     let numbers = vec![10, 25, 30, 55, 60, 75];
//     let result = numbers.iter().find(|x| **x > 50);
//     println!("{:?}", result);
// }

// DRILL 1.16: Any and All
// Problem Statement:
// Check if a vector contains ANY number greater than 100, and if ALL numbers are positive.

// fn main() {
//     let numbers = vec![5, 20, 35, 50, 105];

//     let large = numbers.iter().any(|x| *x > 100);
//     let positive = numbers.iter().all(|x| *x > 0);

//     println!("Has umber > 100 {} All positive: {}", large, positive);
// } 

// DRILL 1.17: Take and Skip
// Problem Statement:
// From numbers 1 to 10, skip the first 3, then take the next 4.

// fn main() {
//     for num in (1..=10).skip(3).take(4) {
//         println!("{}", num);
//     }
// }

// DRILL 1.18: Zip Two Iterators
// Problem Statement:
// Combine two vectors element-by-element and print pairs.

// fn main() {
//     let vec1 = vec!["a", "b", "c"];
//     let vec2 = vec![1, 2, 3];

//     for (letter, number) in vec1.iter().zip(vec2.iter()) {
//         println!("{}: {}", letter, number);
//     }

//     for (x, y) in (1..=5).zip(11..=15) {
//         println!("{} maps to {}", x, y);
//     }

//     for (ch, idx) in "RUST".chars().zip(1..=5) {
//         println!("{}: {}", ch, idx);
//     }
// }

// DRILL 1.19: Flatten Nested Vectors
// Problem Statement:
// You have a vector of vectors. Flatten it into a single sequence and print all numbers.

// fn main() {
//     let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
//     for i in nested.iter().flatten() {
//         println!("{}", i);
//     }
// }

// DRILL 1.20: Complex Chain - The Final Boss
// Problem Statement:
// Combine everything you've learned. Take numbers 1-20, filter odds, square them, skip first 2, take next 3, then sum them.

fn main() {
    let result: i32 = (1..=20).filter(|x| x % 2 != 0).map(|x| x * x).skip(2).take(3).sum();

    println!("{}", result);
}