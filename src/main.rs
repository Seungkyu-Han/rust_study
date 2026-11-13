#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure.trim().to_string();
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let mut silver_chest = TreasureChest {
        captain: String::from("i am silver"),
        treasure: String::from(" ddd "),
    };

    let gold_chest = TreasureChest {
        captain: String::from("Firebard"),
        treasure: ["a", "b", "c"],
    };

    silver_chest.clean_treasure();

    println!("{:?}", silver_chest);

    println!("{}", gold_chest.amount_of_treasure());

    println!("{}", gold_chest.capital_captain());
}
