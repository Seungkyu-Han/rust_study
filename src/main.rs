fn even_or_odd(number: i32) -> String {
    if number % 2 == 0 {
        String::from("even") // 또는 "even".to_string()
    } else {
        String::from("odd")
    }
}

fn main() {
    println!("{}", even_or_odd(4)); // "even"
    println!("{}", even_or_odd(7)); // "odd"
}
