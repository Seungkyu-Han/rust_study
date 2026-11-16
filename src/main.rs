fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("denominator is zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result1 = divide(10.0, 1.0);
    println!("{:?}", result1.is_ok());
    println!("{:?}", result1.unwrap());

    let result2 = divide(10.0, 0.0);
    println!("{:?}", result2.is_ok());
    println!("{:?}", result2.unwrap_or(11.11));
}
