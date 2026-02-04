// enum TrafficLight {
//     Red,
//     Yellow,
//     Green
// }

// fn main() {
//     let light = TrafficLight::Red;

//     match light {
//         TrafficLight::Red => println!("STOP"),
//         TrafficLight::Yellow => println!("Go but Slow"),
//         TrafficLight::Green => println!("Gooo"),
//     }
// }

// enum Message {
//     Quit, // in here no data
//     Move {x: i32, y: i32}, // Named fields like Struct
//     Write(String), // Single value
//     ChangeColor(i32, i32, i32) // Multiple values (like tuple)
// }

// fn process_message(msg: Vec<Message>) {
//     for msgs in msg {
//         match msgs {
//         Message::Quit => println!("Quitting..."),
//         Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
//         Message::Write(text) => println!("Message: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
//     }
//     }

// }

// fn main() {
//     let msg1 = Message::Quit;
//     let msg2 = Message::Move { x: 10, y: 55 } ;
//     let msg3 = Message::Write(String::from("Nigga"));
//     let msg4 = Message::ChangeColor(255, 0, 0);

//     process_message(vec![msg1, msg2, msg3, msg4]);
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// fn main() {
//     let some_number = Option::Some(5);
//     let no_number: Option<i32> = Option::None;

//     match some_number {
//         Option::Some(value) => println!("Got value: {}", value),
//         Option::None => println!("No value"),
//     }
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// fn divide(a: f64, b: f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Result::Err(String::from("Cannot divide by zero"))
//     } else {
//         Result::Ok(a / b)
//     }
// }

// fn main() {
//     match divide(10.0, 2.0) {
//         Result::Ok(result) => println!("Result: {}", result),
//         Result::Err(error) => println!("Error: {}", error),
//     }
//     match divide(10.0, 0.0) {
//         Result::Ok(result) => println!("Result: {}", result),
//         Result::Err(error) => println!("Error: {}", error),
//     }
// }

// enum Shape {
//     Circle(f64),
//     Rectangle(f64, f64),
// }

// impl Shape {
//     fn area(&self) -> f64 {
//         match self {
//             Shape::Circle(radius) => 3.14 * radius * radius,
//             Self::Rectangle(w, h ) => w * h,
//         }
//     }
// }
// fn main() {
//     let circle = Shape::Circle(5.0);
//     let rect = Shape::Rectangle(10.0, 5.0);

//     println!("Circle area: {}", circle.area());
//     println!("Rectangle area: {}", rect.area());
// }

// enum PaymentMethod {
//     Cash,
//     CrediCard { number: String, cvv: u16 },
//     UPI(String),
// }

// fn process_payment(method: PaymentMethod, amout: f64) {
//     match method {
//         PaymentMethod::Cash => {
//             println!("Received {} in cash", amout);
//         }
//         PaymentMethod::CrediCard { number, cvv } => {
//             println!(
//                 "Charging {} to card ending in {}",
//                 amout,
//                 &number[number.len() - 4..]
//             );
//         }
//         PaymentMethod::UPI(id) => {
//             println!("Processing {}, via UPI: {}", amout, id);
//         }
//     }
// }

// fn main() {
//     process_payment(PaymentMethod::Cash, 500.0);
//     process_payment(
//         PaymentMethod::CrediCard {
//             number: String::from("1234556789"),
//             cvv: 123,
//         },
//         15786.2,
//     );
//     process_payment(PaymentMethod::UPI(String::from("user@phonepe")), 6454.4);
// }

// fn main() {
//     let big_box = Box::new([0u8; 1_000_000]);
//     let moved = big_box;

//     println!("{:?}", moved);
// }

// fn main() {
//     let x = 5;
//     let boxed = Box::new(5);

//     println!("x = {}", x);
//     println!("boxed = {}", boxed);
//     println!("boxed = {}", *boxed);
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// } 
// use List::{Cons, Nil};

// fn main() {
//     let list1 = Nil;
//     println!("{:?}", list1);

//     let list2 = Cons(5, Box::new(Nil));
//     println!("{:?}", list2);

//     let list3 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
//     println!("{:?}", list3);
// }

// RC<T> - Reference Counting
//Multiple owners for same data

// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let shared = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("Count after shared: {}", Rc::strong_count(&shared));

//     let list_a = Cons(3, Rc::clone(&shared));
//     println!("Count after list_a: {}", Rc::strong_count(&shared));

//     let list_b = Cons(4, Rc::clone(&shared));
//     println!("Count after list_b: {}", Rc::strong_count(&shared));

//     println!("\nshared: {:?}", shared);
//     println!("list_a: {:?}", list_a);
//     println!("list_b: {:?}", list_b);
// }


// fn main() {
//     let b = Box::new(5);
//     println!("{}", *b);
// }

// our own smart pointer

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T>  {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, *y);
// }


//deref trait
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T>  {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn hello(name: &str) {
//     println!("Hello, {}", name);
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     println!("y = {}", *y);

//     let m = MyBox::new(String::from("ndks"));
//     hello(&m);
// }

// Drop trait

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
//     }
// }

// fn main() {
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };

//     let d = CustomSmartPointer {
//         data: String::from("other stuff"),
//     };
//     println!("CustomerSmartPointers created.");


//     let c = CustomSmartPointer {
//         data: String::from("my data"),
//     };
//     println!("CustomSmartPointer created.");
    
//     drop(c);

//     println!("Dropped before end of main")
// }

// struct CustomerSmartPointer {
//     data: String,
// }

// impl Drop for CustomerSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping: `{}`", self.data);
//     }
// }

// fn main() {
//     let a = CustomerSmartPointer {data: String::from("First")};
//     let b = CustomerSmartPointer {data: String::from("Second")};
//     let c = CustomerSmartPointer {data: String::from("Third")};

//     println!("Created a, b, c");
//     drop(b);  // Early drop

//     println!("b droped early");
//     println!("End of main...");
//     //a and c drop here automatically

// }

/// RefCell<T> Interior Mutability

// fn main() {
//     let mut x = 4;
//     let y = &mut  x;
// }

use std::cell::RefCell;
use std::rc::{Rc, Weak};
// struct Cache {
//     data: RefCell<Vec<String>>, //Mutable inside immutable
// }

// impl Cache {
//     fn new() -> Cache {
//         Cache { data: RefCell::new(vec![]) }
//     }
//     // &self is immutable but we can still modify the data
//     fn add(&self, item: String) {
//         self.data.borrow_mut().push(item); // Mutate
//     }

//     fn get_count(&self) -> usize {
//         self.data.borrow().len() // Read only
//     }
// }

// fn main() {
//     let cache = Cache::new(); // Immuatable

//     cache.add(String::from("item1"));
//     cache.add(String::from("item2"));

//     println!("Count: {}", cache.get_count())
// }

// fn main() {
//     let value = RefCell::new(5);

//     println!("Original: {:?}", value);

//     *value.borrow_mut() += 10;

//     println!("After mutation: {:?}", value);

//     let a = value.borrow();
//     let b = value.borrow();

//     println!("a = {}, b = {}", *a, *b);
// }

#[derive(Debug)]

// struct Node {
//     value: i32,
//     next: RefCell<Option<Rc<Node>>>,
// }

// fn main() {
//     let a = Rc::new(Node {value: 1, next: RefCell::new(None)});
//     let b = Rc::new(Node {value: 2, next: RefCell::new(None)});

//     *a.next.borrow_mut() = Some(Rc::clone(&b));
//     *b.next.borrow_mut() = Some(Rc::clone(&a));

// }

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Parent = Weak )no ownership
    children: RefCell<Vec<Rc<Node>>> // children = Rc (ownership)
}

fn main() {
    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);

    println!("Strong count: {}", Rc::strong_count(&strong));
    println!("Weak count: {}", Rc::weak_count(&strong));


    //accessing weak reference
    if let Some(value) = weak.upgrade() {
        println!("Value: {}", value);
    }

    drop(strong); // Strong reference is drop GONE

    // Weak cant access anymore
    println!("After drop: {:?}", weak.upgrade()); // None
}