fn main() {
    let age: i32 = 33;

    {
        let is_handsome: bool = true;

        println!("is handsome is {is_handsome}");
    }

    println!("age is: {age}");
}
