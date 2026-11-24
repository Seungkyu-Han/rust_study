use std::fmt::{Display, Formatter};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "apple for apple")
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: String::from("Granny Smith"),
        price: 1.05,
    };

    println!("{}", lunch_snack);
}
