fn main() {
    let earnings = [4, 7, 9, 13];

    let sum = earnings.into_iter().fold(0, |total, current| {
        total + current
    });

    println!("{sum}");
}