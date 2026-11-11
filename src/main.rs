fn main() {
    let food = "pizza";
    println!("{}", food.len());
    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());
}
