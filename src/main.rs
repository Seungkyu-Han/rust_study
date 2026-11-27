fn main() {
    let seafood = "Oyster ";

    for byte in seafood.bytes() {
        print!("{byte}");
    }

    println!("{seafood}");
}
