use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        let count = counts.entry(word).or_insert(0);

        *count += 1;
    }

    counts
}

fn main() {
    println!("{:?}", count_words("Hello, What is your name"));
}
