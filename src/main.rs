fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let my_iter = numbers.iter();

    let squares: Vec<String> = my_iter.map(|s| s.to_string()).collect();

    println!("{squares:?}");

    for number in squares {
        println!("{number}");
    }
}
