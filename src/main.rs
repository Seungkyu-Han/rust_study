fn main() {
    let numbers: Vec<i32> = vec![4, 8, 15, 16, 23, 42];

    let squares: Vec<i32> = numbers.iter().map(|&number| number.pow(2)).collect();
    println!("{squares:?}");

    for number in squares {
        println!("{number}");
    }
}
