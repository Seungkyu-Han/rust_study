use std::collections::HashMap;

fn main() {
    let mut coffee_pairings = HashMap::new();

    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_pairings.insert(&drink, &milk);

    println!("{:?}", coffee_pairings.len());

    println!("{:?}", drink);
}
