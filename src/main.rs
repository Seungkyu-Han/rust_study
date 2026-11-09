fn main() {

    let mut seconds: i32 = 10;

    loop {
        println!("{seconds} seconds to blastoff...");
        seconds -= 1;
        if seconds == 0 {
            break;
        }
    }
}
