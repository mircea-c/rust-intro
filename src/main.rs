fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V6,
        V4,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home);
    println!("{:?}", loopback);

    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("Hello, I am Baymax!"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let no_number: Option<i32> = None;
    plus_one(some_number);
    plus_one(no_number);

    if let Some("a string") = some_string {
        println!("Strings are some!")
    }

    // There's more to this in chapter 18
    match m {
        Message::Quit => println!("QUIT!"),
        Message::Move {..} => println!("NO"),
        Message::ChangeColor(..) => println!("RED"),
        Message::Write(..) => println!("Falalalalalala!"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
