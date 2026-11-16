fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instruments.get(2);

    match bass {
        Option::Some(instrument) => {
            println!("Playing the {instrument}")
        }
        Option::None => {
            println!("Singing with my voice");
        }
    }

    let invalid_instrument = musical_instruments.get(100);

    match invalid_instrument {
        Option::Some(instrument) => {
            println!("Playing the {instrument}")
        }
        Option::None => {
            println!("Singing with my voice");
        }
    }
}
