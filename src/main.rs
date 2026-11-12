#[derive(Debug)]
struct Song {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl Song {
    fn display_song_info(&self) {
        println!(
            "{} {} {}",
            self.title, self.release_year, self.duration_secs
        );
    }

    // 더 관용적인(idiomatic) 표현
    fn double_length(&mut self) {
        self.duration_secs *= 2;
        println!("{:#?}", self);
    }
}

fn main() {
    let mut song = Song {
        title: String::from("Hello Song"),
        release_year: 2025,
        duration_secs: 123,
    };

    song.double_length();

    song.display_song_info();

    //println!("{:?}", song);
}
