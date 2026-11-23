use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("steak"), 29.99);
    menu.insert(String::from("tuna"), 29.99);
    menu.insert(String::from("burger"), 14.99);

    println!("{menu:?}");

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();

    country_capitals.insert("france", "Paris");
    country_capitals.insert("Germany", "Berlin");

    println!("{country_capitals:?}");
}
