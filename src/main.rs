fn main() {
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text1 = "wow";
    let text_as_number1 = text1.parse::<i32>();
    println!("{:?}", text_as_number1);
}
