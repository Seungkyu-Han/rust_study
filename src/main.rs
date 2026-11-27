fn main() {
    let fifty_numbers = 1..=50;

    for number in fifty_numbers.rev().skip(10).take(15).step_by(4) {
        print!("{number}/");
    }
}
