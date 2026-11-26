fn main() {
    let multiplier = 5;

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };


    println!("{}", multiply_by(10));
}