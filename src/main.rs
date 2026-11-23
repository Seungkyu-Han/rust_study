use std::collections::HashMap;

trait Accomodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: String) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl Accomodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: Vec::new(),
        }
    }
}

impl Accomodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

fn main() {
    let mut hotel = Hotel::new(String::from("Seungkyu"));
    println!("{:#?}", hotel.get_description());

    hotel.book("Pires", 11);
    println!("{:#?}", hotel);

    let mut airbnb = AirBnB::new("peter");
    airbnb.book("John", 5);

    println!("{:?}", airbnb.get_description());
}
