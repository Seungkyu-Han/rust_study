fn main() {
    let cake = bake_cake();

    println!("{} is cake", cake);
}

fn bake_cake() -> String {
    String::from("Cake")
}