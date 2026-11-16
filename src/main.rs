fn main() {
    let present = Option::Some(13);
    let missing_value = Option::None;

    println!("{}", present.unwrap_or(111));
    println!("{}", missing_value.unwrap_or(111));
}
