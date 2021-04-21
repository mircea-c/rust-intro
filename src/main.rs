fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else if number % 2 == 0 {
        //Rust won't try to convert other types to bool. Be explicit
        println!("number divisible by 2");
    } else {
        println!("false");
    }

    let condition = true;
    //both branches of an if have to be the same type
    let number = if condition { 5 } else { 6 };
    print!("The value of number is: {}\n", number);

    // interesting, loops return a value!!!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("AGAIN!");
        if counter == 10 {
            break counter * 2;
        } // why no need for semicolon here?!?!?
    };

    print!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LOL");

    // My fav loop now, why even have the others :p
    let a =[10,20,30,50,07];
    for elem in a.iter() {
        println!("the value is {}", elem);
    }
}
