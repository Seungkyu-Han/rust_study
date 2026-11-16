fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(0);

    println!("{:?}", bass);

    let invalid_instrument = musical_instruments.get(100);

    println!("{:?}", invalid_instrument);
}
