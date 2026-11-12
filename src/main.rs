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
        println!("Years since release: {}", self.years_since_release());
    }

    // 더 관용적인(idiomatic) 표현
    fn double_length(&mut self) {
        self.duration_secs *= 2;
        println!("{:#?}", self);
    }

    fn is_longer_than(&self, other: &Self) -> bool {
        self.duration_secs > other.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        4000 - self.release_year
    }
}

fn main() {
    let mut song1 = Song {
        title: String::from("Hello Song1"),
        release_year: 2025,
        duration_secs: 123,
    };

    let mut song2 = Song {
        title: String::from("Hello Song2"),
        release_year: 2025,
        duration_secs: 123,
    };

    song1.double_length();

    song1.display_song_info();

    let result = song1.is_longer_than(&song2);

    println!("{}", result);

    //println!("{:?}", song);
}
