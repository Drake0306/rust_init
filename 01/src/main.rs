use std::collections::HashMap;

fn main() {
    // dataTypes
    let unsigned: u8 = 10;
    let signed: i8 = -10;
    let float:f32 = 1.2;

    println!("Unsigned: {} Sign: {} Float: {}", unsigned, signed, float);

    let letter = "c";
    let emoji = "\u{1F600}";

    println!("letter: {}, enoji: {}", letter, emoji);
    let is_true: bool = true;

    println!("isTrue: {}", is_true);


    // Array
    let arr:[u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());
    println!("{:?}", other_arr);


    // Tuple
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple;

    // deconstructing
    println!("first {}, second {}, third {}", a, b, c);


    // Function
    println!("{}", is_even(2));


    // slice and array
    let arr = [0, 1, 2, 3]; // length
    let slice = &arr[1 .. 3]; // [1, 2] don't know the length

    borrowing_slice(arr, slice);


    // String
    let str: &str = "Hello world";
    let mut string: String = String::from("Hello world");

    let slice_one = &string[.. 6];
    println!("length {}", slice_one.len());
    
    string.push('1');
    string.push_str("! Bob");
    string = string.replace("Hello", "Bye");
    println!("{}", string);


    // IF Statement
    let n = 3;
    if n > 0 {
        println!("grater than 0");
    } else if n < 0 {
        println!("less than 0");
    } else {
        println!("is 0");
    }


    // For loop
    for i in 0..6 {
        println!("{}", i);
    }

    // while loop
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exit");
            break

            // break
            // continue
        }
    }


    //Match ! simmilar to switch but with more flexiblity
    let i= 5;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("default")
    }

    
    // Structs
    let name  = String::from("Bird");
    let bird = Bird { name: name, attack: 5};
    bird.print_name();


    // Traits
    println!("{} {}", bird.can_fly(), bird.is_animal());

    // enums
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C{x:10, y:20};

    if let MyEnum::B(val) = b {
        println!("{}", val);
    }

    if let MyEnum::C { x, y } = c {
        println!("{} {}", x, y);
    }


    // Vector
    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);


    // Maps
    let mut map = HashMap::new();

    map.insert(0, "hi");
    map.insert(1, "hi2");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("{}", str), // some or 
        _=> println!("Dosen't exist in map"), // none
    }

    match map.get(&2) {
        Some(str) => println!("{}", str), // some or 
        _=> println!("Dosen't exist in map"), // none
    }

    map.remove(&0);
    println!("{:?}", map);


    // Option
    let divide1: Option<i32> = divide(4,2);
    let divide2: Option<i32> = divide(2,3);

    println!("{:?} unraps to {}",divide1, divide1.unwrap());
    // println!("{:?} unraps to {}",divide2, divide2.unwrap()); // Unrapping a None type will throw a run time error

}

//functions
pub fn is_even(num: u8) -> bool { 
    let digit: u8 = num % 2;
    digit == 0 // return value
}

// slice and array
fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}


// Structs
struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{} , {}", self.name, self.attack);
    }
}


// trait
impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }

    fn is_animal(&self) -> bool {
        false
    }
}
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}


// enums
#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}


// Option
fn divide (dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

