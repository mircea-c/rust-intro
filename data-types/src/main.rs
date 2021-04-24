use std::io;

fn main() {
    // 'tuples' they say. tuples all the way down
    // can store different types in a tuple
    let tup = (500, 6.4, "string");
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    // Accessing using '.' notation
    // chars literals use single quotes
    let x: (i32, f64, char) = (500, 6.4, 'z');
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);

    // Arrays are tuples that can only hold a single type of value
    // Fixed length always
    // Allocated on the stack rather than the heap
    let a = ["yogi", "bear", "is a foodie"];
    // A 5 element array with all the elements having the same value
    let b = [3; 5];

    println!("Enter an array index, NOW!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element_a = a[index];
    let element_b = b[index];

    println!(
        "The value of element from array a at index {} is {}",
        index, element_a
    );

    println!(
        "The value of element from array b at index {} is {}",
        index, element_b
    );
}