#[derive(Debug, Copy, Clone)]
enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    // 2. Take `self` (ownership) and return `T` (not `<T>`)
    fn unwrap(self) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("wow"),
        }
    }

    fn unwrap_or(self, fallback_value: T) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn main() {
    let some_option = MyOption::Some(100);

    println!("{:?}", some_option.unwrap());

    let some_none: MyOption<bool> = MyOption::None;

    println!("{:?}", some_none.unwrap_or(true));
}
