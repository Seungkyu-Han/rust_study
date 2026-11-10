fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    current_meal =  add_sugar(current_meal);
    current_meal =  add_salt(current_meal);

    println!("{}", current_meal);
}

fn add_flour(mut meal: String) -> String{
    meal.push_str(" with flour");
    meal
}


fn add_sugar(mut meal: String) -> String{
    meal.push_str(" with sugar");
    meal
}


fn add_salt(mut meal: String) -> String{
    meal.push_str(" with salt");
    meal
}