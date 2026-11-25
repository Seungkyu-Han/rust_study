struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.destination == other.destination
            && self.time == other.time
    }
}

fn main() {
    let a = Flight::new("a", "b", "c");
    let b = Flight::new("a", "b", "c");

    println!("{}", a == b);
    println!("{}", a != b);
}
