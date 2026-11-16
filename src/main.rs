fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instruments.get(0);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("nothing");

    println!("{:?}", valid_instrument);

    let invalid_instrument = musical_instruments.get(100);
    invalid_instrument.expect("nothing1");
}
