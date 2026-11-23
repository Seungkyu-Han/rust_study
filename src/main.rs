use std::io;

fn main() {
    let mut name = String::new();
    println!("What is your name?");
    let result = io::stdin().read_line(&mut name);
    match result {
        Ok(value) => println!("size is {value}"),
        Err(_) => println!("this is error"),
    }
    println!("Hello, {name}");
}
