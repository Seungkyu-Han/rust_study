#[derive(Debug)]
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    if let Milk::Whole = my_beverage {
        println!("You have whole milk");
    }
}
