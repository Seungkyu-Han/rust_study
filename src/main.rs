use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();

    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    println!("{:?}", concert_queue.union(&movie_queue));
    println!("{:?}", concert_queue.difference(&movie_queue));
}
