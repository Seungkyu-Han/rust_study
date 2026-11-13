#[derive(Debug)]
enum CheeseSteak<T> {
    Plain,
    Topping(T),
}

fn main() {
    let mushroom = CheeseSteak::Topping("mushroom");
    let onions = CheeseSteak::Topping("onions".to_string());
    let topping = "bacon".to_string();
    let bacon = CheeseSteak::Topping(&topping);

    let plain: CheeseSteak<String> = CheeseSteak::Plain;

    println!("{:?}", plain);
}
