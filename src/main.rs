use std::fmt::{Debug, Display, Formatter, Result};

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
