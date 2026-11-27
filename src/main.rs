fn main() {
    let applicants = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];

    let winners: Vec<&str> = applicants
        .into_iter()
        .enumerate()
        .map(|(index, s)| {
            println!("{index} {s}");
            s
        })
        .collect();

    println!("{winners:?}");
}
