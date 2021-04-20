fn main() {
    // variables are immutable by default
    let y = 2;

    println!("The vaule of y is: {}", y);

    // use the mut keyword to make variables mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants *must* be annotated
    // valid in any scope
    // funny underscore in numeric literals
    const LEET_POINTS: u32 = 100_000;
    println!("Constant is: {}", LEET_POINTS);

    // 'shadowing' changing the value (and type too!) of an immutable variable
    let y = y.to_string() + " and y is a string";
    println!("The value of y is {}", y);
}
