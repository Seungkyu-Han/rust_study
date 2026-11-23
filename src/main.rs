use std::{
    fs::File,
    io::{Read, stdin},
};

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

    let mut file = File::open(input.trim()).expect("hello");

    input.clear();

    let read_operation = file.read_to_string(&mut input);

    println!("{input}");
}
