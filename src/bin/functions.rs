// DRILL 3.1: Basic Function with Parameters
// Problem Statement:
// Create a function that takes two numbers and returns their sum.

// fn add(a: i32 , b: i32) -> i32 {
//     a + b
// }
// fn main () {
//     println!("{}", add(10, 20));
// }

// DRILL 3.2: Function with No Return (Unit Type)
// Problem Statement:
// Create a function that prints a greeting but doesn't return anything.

// fn greet(name: &str) {
//     print!("Hello, {name}!")
// }
// fn main() {
//     greet("Rust");
// }

// DRILL 3.3: Early Return with Validation
// Problem Statement:
// Create a function that divides two numbers, but returns early with an error message if dividing by zero.

// fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         Err("Cannt divide by zero".to_string())
//     } else {
//         let result = a / b;
//         Ok(result)
//     }
// }

// fn main() {
//     let result = safe_divide(10, 0);
//     if let Ok(value) = result {
//         println!("Result: {}", value);
//     } else if let Err(err) = result {
//         println!("Result: {}", err)
//     }
// }

// DRILL 3.4: Recursion - Factorial
// Problem Statement:
// Create a recursive function that calculates factorial of a number.

// fn factorial(n: u32) -> u32 {
//     if n <= 1 {
//         1
//     } else {
//         n * factorial(n - 1) // this is where magic is
//     }
// } 

// fn main() {
//     println!("{}", factorial(5));
// }

// DRILL 3.5: Closures - Anonymous Functions
// Problem Statement:
// Create a closure (anonymous function) that squares a number, then use it to transform a vector.
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5];

//     let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
//     println!("{:?}", squared);
// }

// DRILL 3.6: Higher-Order Functions
// Problem Statement:
// Create a function that takes another function as a parameter and applies it to a number.

// fn double(n: i32) -> i32 {
//     n * 2
// }
// fn triple(n: i32) -> i32 {
//     n * 3
// }
// fn apply_operation(x: i32, operation: fn(i32)-> i32) -> i32 {
//     operation(x)
// }
// fn main() {
//     let a = apply_operation(5, double);
//     let b = apply_operation(5, triple);

//     println!("{}", a);
//     println!("{}", b)
// }

// DRILL 3.7: Closures vs Functions - Capturing Environment
// Problem Statement:
// Demonstrate the key difference between closures and regular functions: closures can capture variables from their surrounding scope.

// fn main() {
//     let multiplier = 10;
//     let number = vec![1, 2, 3];
//     let result: Vec<i32> = number.iter().map(|x| x * multiplier).collect();

//     println!("{:?}", result);
// }

// DRILL 3.8: Method Syntax
// Problem Statement:
// Learn the difference between associated functions (like String::new()) and methods (like my_string.len()). Implement a simple struct with both.

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
// }

// fn main() {
//     let rectangle = Rectangle::new(5, 10);
//     let result = rectangle.area();
//     println!("Area of Rectangle: {}", result);
    
// }


// DRILL 3.9: Returning Closures
// Problem Statement:
// Functions can return closures. Create a function that returns a closure which adds a specific value to its input.

// fn make_addr(x: i32) -> impl Fn(i32) -> i32 {
//     move |val| val + x
// }
// fn main() {
//     let add5 = make_addr(5);
//     let add10 = make_addr(10);

//     println!("{}", add5(20));
//     println!("{}", add10(20));
// }


// DRILL 3.10: Generic Functions
// Problem Statement:
// Make functions work with multiple types using generics. Create a generic function that finds the largest item in a slice.

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for num in list {
        if num > &largest {
            largest = *num
        }
    }
    return largest;
}

fn main() {
    let numbers = vec![3, 5, 1, 8, 2];
    let chars = vec!['a', 'z', 'm', 'b'];

    println!("Largest num: {}", largest(&numbers));
    println!("Largest chars: {}", largest(&chars));
}