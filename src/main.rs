use rust_study::lodging::{Accomodation, AirBnB, Description, Hotel};
use rust_study::utils::{book_for_one_night, mix_and_match};

fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    println!("{}", hotel.summarize());
    hotel.book("Dana", 5);

    let mut airbnb = AirBnB::new("Parker");
    println!("{}", airbnb.get_description());
    book_for_one_night(&mut airbnb, "Dan");

    mix_and_match(&mut hotel, &mut airbnb, "Phil");
}
