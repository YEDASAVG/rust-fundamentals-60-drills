// DRILL 2.1: Basic If Statement
// Problem Statement:
// Check if a number is positive, negative, or zero using if/else

// fn main() {
//     let num = -5;
//     if num > 0 {
//         println!("Positive");
//     } else if num < 0 {
//         println!("Negative");
//     } else {
//         println!("Zero")
//     }
// }

//  DRILL 2.2: If as Expression
// Problem Statement:
// Use if as an expression to assign a value based on a condition.

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <score>", args[0]);
//         std::process::exit(1);
//     }
//     let score_str = &args[1];
//     let score: i32 = score_str.parse().expect("Failed to parse the input");

//     let grade = if score >= 90 {
//         "A"
//     } else if score >= 80 {
//         "B"
//     } else if score >= 70 {
//         "C"
//     } else {
//         "F"
//     };
//     println!("Grade: {}", grade);
// }

// DRILL 2.3: Match with Enums
// Problem Statement:
// Create a traffic light enum and print actions based on the light color.

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green
// }
// fn main() {
//     let light = TrafficLight::Yellow;
//     match light {
//         TrafficLight::Red => println!("Stop"),
//         TrafficLight::Yellow => println!("Slow Down"),
//         TrafficLight::Green => println!("Goo"),
//     }
// }


// DRILL 2.4: Match with Numbers
// Problem Statement:
// Use match to categorize a number into ranges.

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         eprintln!("Usage: {} <num>", args[0]);
//         std::process::exit(1);
//     }
//     let num_str = &args[1];
//     let num: i32 = num_str.parse().expect("Failed to parse the input"); 

//     match num {
//         1..=10 => println!("Small"),
//         11..=50 => println!("Medium"),
//         51..=100 => println!("Large"),
//         _ => println!("Out of range"),
//     }
// }

// DRILL 2.5: Match Guards
// Problem Statement:
// Use match with guards (additional conditions) to check if a number is even or odd AND in a specific range.

// fn main() {
//     let num = 24;
//     match num {
//         n if n >= 20 && n <= 30 && n % 2 == 0 => println!("Even in range"), 
//         n if n >= 20 && n <= 30 && n % 2 != 0 => println!("Odd in range"),
//         _ => println!("Out of range"), 
//     }
// }

// DRILL 2.6: Match with Tuples
// Problem Statement:
// Match on a tuple to determine a person's status based on age and employment.

// fn main() {
//     let person = (25, true); //(age, is_employed)

//     match person {
//         (18..=25, true) => println!("Young Professional"),
//         (18..=25, false) => println!("Young Job-Seeker"),
//         (26..=65, true) => println!("Experienced Professional"),
//         (26..=65, false) => println!("Job - Seeker"),
//         _ => println!("Retired or young age"),
//     }
// }

// DRILL 2.7: Match with Option
// Problem Statement:
// Use match to handle an Option<i32> value safely.

// fn main () {
//     let maybe_num: Option<i32> = Some(25);
//     match maybe_num {
//         Some(val) => println!("Got: {val}"),
//         None => println!("nothing here"),
//     }
// }

// DRILL 2.8: Match with Result
// Problem Statement:
// Use match to handle a Result from parsing a string to number.

// fn main() {
//     let text = "56";
//     let result = text.parse::<i32>();

//     match result {
//         Ok(num) => println!("Parsed: {num}"),
//         Err(e) => println!("Failed: {e}"),
//     }
// }

// DRILL 2.9: Nested Match
// Problem Statement:
// Match on nested Option values (Option within Option).

// fn main() {
//     let nested = Some(Some(10));
//     match nested {
//         Some(Some(n)) => println!("Double wrapped: {}", n),
//         Some(None) => println!("Outer Only"),
//         None => println!("Nothing"),
//     }
// }

// DRILL 2.10: If Let - The Final Boss
// Problem Statement:
// Use if let as a shorthand when you only care about one pattern.

fn main() {
    let value = Some(100);

    if let Some(x) = value && x > 50 {
        println!("Large value: {}", x);
    }
}
