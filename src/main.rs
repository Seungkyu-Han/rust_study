#[derive(PartialEq)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongWriter};

fn main() {
    let rustin_bieber = SingerSongWriter("Rustin".to_string());

    let rust_no_one = Band(5);

    println!("{}", rustin_bieber == rust_no_one);
}
