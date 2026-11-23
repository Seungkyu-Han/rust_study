use std::collections::HashMap;

fn main() {
    let mut coffee_pairings: HashMap<&str, &str> = HashMap::new();

    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);
    coffee_pairings.insert("Flat white", "Almond milk");

    let value1 = coffee_pairings["Flat white"];
    let value2 = coffee_pairings.get("Flat white");

    println!("{}", value1);
    println!("{:?}", value2);
}
