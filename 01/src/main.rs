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


}

//functions
pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return value
}


fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
