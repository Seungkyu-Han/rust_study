use std::fs::File;

fn main() {
    let file = match File::open("story.txt") {
        Ok(value) => value,
        Err(_err) => panic!(),
    };

    println!("{file:#?}");
}
