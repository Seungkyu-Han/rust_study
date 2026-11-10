fn main() {
    let apples: i32 = 5;
    print_my_value(apples);
    println!("{apples} is still valid");

    let oranges: String = String::from("Oranges");
    println!("{oranges} is valid");
    print_my_string_value(oranges);
    //println!("{oranges} is still valid");

}

fn print_my_value(value: i32) {
    println!("The value is: {}", value);
}

fn print_my_string_value(value: String) {
    println!("The value is: {}", value);
}