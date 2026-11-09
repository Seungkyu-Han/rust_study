fn main() {
    let evaluation: i32 = 100;

    let value = match evaluation {
        1 => 20,
        _ => 30,
    };

    println!("{}", value);
}
