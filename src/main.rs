fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let total: i32 = numbers.iter().sum();

    println!("{total}");

    let product: i32 = numbers.iter().product();

    println!("{product}");

    println!("{:?}", numbers.iter().count());
}
