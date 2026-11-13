enum LanundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LanundryCycle {
    fn wash_laundry(&self) {
        match self {
            LanundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            }
            LanundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LanundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}");
            }
        }
    }
}

fn main() {
    LanundryCycle::Cold.wash_laundry();
    LanundryCycle::Hot { temperature: 100 }.wash_laundry();
    LanundryCycle::Delicate(String::from("silk")).wash_laundry();
}
