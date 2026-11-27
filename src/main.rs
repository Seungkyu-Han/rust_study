fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();

    for number in my_vector.iter() {
        println!("{number}");
    }

    println!("{my_iterator:?}");

    let cities = vec![String::from("Phoenix"), String::from("Dallas")];

    for city in &cities {
        println!("{city}");
    }

    println!("{cities:?}");
}
