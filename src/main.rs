fn operation(great_success: bool) -> Result<String, &'static str> {
    if great_success {
        Ok("Success".to_string())
    } else {
        Err("Failure")
    }
}

fn main() {
    let my_result = operation(false);

    let content = match my_result {
        Ok(message) => message,
        Err(error) => error.to_string(),
    };

    println!("{}", content);
    //println!("{:?}", my_result.unwrap());
}
