fn main() {
    let pepperoni = String::from("pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    let mut delicious_toppings = pizza_toppings;

    delicious_toppings.push(String::from("Olives"));

    println!("{delicious_toppings:?}");
}
