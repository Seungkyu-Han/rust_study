struct ShortDuration(u32, u32);

struct LongDuration(u32, u32);

fn main() {
    let work_shift = ShortDuration(8, 0);
    println!("{} hours {}", work_shift.0, work_shift.1);
}
