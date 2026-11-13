#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Firebard"),
        treasure: ["a", "b", "c"],
    };

    println!("{:?}", gold_chest);
}
