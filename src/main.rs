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
}
