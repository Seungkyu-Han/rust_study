fn main() {
    let a: Option<i8> = Option::Some(1);
    let b = Option::Some("hello");
    let c = Option::Some(true);
    let d = Option::<i16>::Some(5);
    let mut e: Option<i32> = Option::None;

    e = Option::Some(1);
}
