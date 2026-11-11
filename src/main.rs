fn main() {
    let mut coffee: String = String::from("Mocha");
    let a = &mut coffee;
    let b = a;

    println!("and {b}");
}
