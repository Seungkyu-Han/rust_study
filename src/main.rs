struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious: {}", coffee.name);
    coffee.is_hot = false;
}

fn main() {
    let mut mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);

    drink_coffee(&mut mocha);

    println!("{} coffee is {} hot", mocha.name, mocha.is_hot);
}
