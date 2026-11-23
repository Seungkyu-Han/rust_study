use std::{fs::File, io::stdin};

fn main() {
    println!("Please enter the name of the file you'd like to read: ");

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(value) => {
            println!("input length is {value}")
        }
        Err(_err) => {
            panic!("input error");
        }
    };

    let file = File::open(input.trim());

    println!("{:?}", file);
}
