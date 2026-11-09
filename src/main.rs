fn main() {

    let mut seconds: i32 = 21;

    while seconds > 0 {
        seconds -= 1;

        if seconds % 2 == 0 {
            continue;
        }

        println!("{seconds} seconds to blastoff...");
    }
}
