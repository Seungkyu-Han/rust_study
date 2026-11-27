fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();

    println!("{more_teas:?}");
}
