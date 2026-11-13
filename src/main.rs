fn main() {
    let musical_instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let third = musical_instruments.get(2);
    let fourth: Option<&String> = musical_instruments.get(3);

    println!("{:?}", third);
    println!("{:?}", fourth);
}
