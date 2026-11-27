fn main() {
    let performers = ["Rustful Five", "Rustful in Peace", "Rustful in Bieber"];

    let last = performers.into_iter().last().unwrap();

    println!("{last:?}");

    let second = performers.into_iter().nth(2).unwrap();

    println!("{second:?}");
}
