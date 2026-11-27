fn main() {
    let attendees = ["A, B, C", "D, E, F", "G, H, I"];

    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();

    println!("{attendees:?}");
}
