fn main() {
    let mut seconds: i32 = 10;

    loop {
        seconds -= 1;

        if seconds % 2 == 0 {
            continue;
        }

        println!("{seconds} seconds to blastoff...");
        if seconds <= 0 {
            break;
        }
    }
}
