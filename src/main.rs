struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = Coffee {
        price: 2.5,
        name: String::from("mocha"),
        is_hot: true,
    };

    println!("Mocha name: {}", mocha.name);
    println!("Mocha price: {}", mocha.price);
    println!("Mocha is hot: {}", mocha.is_hot);

    let favorite_coffee = mocha.name.clone();

    println!("Mocha name: {}", mocha.name);
    println!("Favorite coffee: {}", favorite_coffee);
}
