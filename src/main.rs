use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("sk");

    if concert_queue.contains("sk") {
        println!("i am sk");
    }

    concert_queue.insert("sk");

    println!("{concert_queue:?}");

    println!("{:?}", concert_queue.remove("sk"));
}
