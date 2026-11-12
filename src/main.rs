#[derive(Debug)]
struct Song {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl Song {
    fn display_song_info(self: Self) {
        println!(
            "{} {} {}",
            self.title, self.release_year, self.duration_secs
        );
    }
}

fn main() {
    let song = Song {
        title: String::from("Hello Song"),
        release_year: 2025,
        duration_secs: 123,
    };

    song.display_song_info();

    //println!("{:?}", song);
}
