fn main() {
    let mut s = String::from("hello"); // allocated on the heap
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    // pushing things on the stack is cheap and easy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s2 = s; // this creates a copy of the s pointer. Heap data is the same
                // but really this is a lie. It's not a copy, it's a MOVE
                // s IS NO LONGER VALID HERE!!! so you'll get a compile error if you try to use it
                // s2 is king now
                // println!("yikes s value is: {}", s);

    // if you really need a copy!
    let s3 = s2.clone(); // it's clear that this is expensive, cloning requires vespene gas
    println!("s2 = {}, s3 = {}", s2, s3);

    takes_ownership(s3); // s3 becomes invalid here
                         // println!("{}", s3); // won't work

    makes_copy(y); // this uses `Copy` so it's cool, can still use y
    println!("y after is still {}", y);

    // let's give back what was taken
    let s2 = takes_and_gives_back(s2);
    println!("s2 is still alive:{}", s2);

    // this ownership dance is tiring...
    // let's try references
    let len = sane_ownership(&s2); // &s2 is the reference (fancy name for pointer)
    println!("len is {}", len);

    // Of course there's also mutable references to mutable variables
    let mut s_again = String::from("Hello, I am Baymax!");
    change(&mut s_again);
    println!("{}", s_again);

    let sr1 = &mut s_again; // one is company
    // let sr2 = &mut s_again; // more than one will ruin your day
    // println!("{}, {}", sr1, sr2);
    println!("{}", sr1);

    // Just gonna copy paste something here
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // if r1 and r2 are not used ever, compiler will warn
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // slice and dice
    println!("{}", first_word("baymax"));
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("y copy is {}", some_integer);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn sane_ownership(s: &String) -> usize {
    // s is not mutable here, so we can't do something like
    // s.push_str(", BAD");
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" Falalalala!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}