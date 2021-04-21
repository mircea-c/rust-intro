fn main() {
    println!("Hello, world!");

    println!("Will this work? {}", another_function(5, 6));
}

fn another_function(x: i32, y: i32) -> i32 {


    let z = {
        let x = 3;
        x + y
    };

    // This returns... that's just weird man...
    z + x
}
