fn main() {
    let option = Some("Salami");

    let food = option.unwrap_or_else(|| "pizza");
    println!("{food}");

    let option: Option<&str> = None;
    let pizza_fan = false;
    let food = option.unwrap_or_else(|| if pizza_fan { "Pizza" } else { "Hot potatao" });

    println!("{food}");
}
