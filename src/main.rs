use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => writeln!(formatter, "DDDelicious"),
            AppleType::GrannySmith => writeln!(formatter, "GGGrany"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => writeln!(f, "DDDelicious"),
            AppleType::GrannySmith => writeln!(f, "GGGrany"),
        }
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("apple is removed"),
            Err(_) => println!("apple is error"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

fn main() {
    let lunck_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.05,
    };

    println!("{}", lunck_snack);
}
