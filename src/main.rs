struct Baymax {
    shape: String,
    armor_type: String,
    charge_level: u64,
    active: bool,
}

#[derive(Debug)] // derive the debug trait required to be able to print
struct Rectangle {
    width: u32,
    height: u32,
}

// neat and tidy. I like!
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wat(self) {
        // this is crazytown...
        println!("methods can take ownership of the struct")
    } // so self (the whole struct) stops being valid here?!?!
      // wonder if this is ever used in practice

    fn can_hodl(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associates
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let bot = Baymax {
        shape: String::from("puffy"),
        armor_type: String::from("Ultra"),
        charge_level: 64,
        active: true,
    };

    println!("{}", bot.charge_level);
    // to update a field, the whole struct must be mutable

    let minimax = build_baymax(String::from("small"), String::from("baby"));
    println!("{}", minimax.charge_level);

    let rect = Rectangle {
        width: 420,
        height: 69,
    };

    println!("Area of the rectangle is {} square pixels.", area(&rect));

    println!("rectangle is {:?}", rect);

    println!(
        "Now we're using methods for the area, which is {}",
        rect.area()
    );
    rect.wat(); // rect ends here since it's being moved

    let rect2 = Rectangle {
        height: 420,
        width: 69,
    };

    let rect3 = Rectangle {
        height: 66,
        width: 66,
    };

    println!("Can rect2 hodl rect3? {}", rect2.can_hodl(&rect3));
    println!("Can rect3 hodl rect2? {}", rect3.can_hodl(&rect2));

    let sq = Rectangle::square(5);
    println!("Square area is {}", sq.area());
}

fn build_baymax(shape: String, armor_type: String) -> Baymax {
    Baymax {
        shape,
        armor_type,
        active: true,
        charge_level: 54,
    }
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
