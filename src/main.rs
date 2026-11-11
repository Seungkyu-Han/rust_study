struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mut beverage = Coffee {
        price: 2.5,
        name: String::from("mocha"),
        is_hot: true,
    };

    beverage.name = String::from("latte");

    println!(
        "My {} this morning cost {}. It is {} that it was hot",
        beverage.name, beverage.price, beverage.is_hot
    );
}
