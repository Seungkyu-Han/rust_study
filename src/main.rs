fn main() {
    let mut pizza_diameters = vec![8, 10, 12, 14];
    pizza_diameters.push(16);
    pizza_diameters.insert(0, 6);

    println!("{pizza_diameters:?}");

    pizza_diameters.pop();
    pizza_diameters.remove(0);

    println!("{pizza_diameters:?}");
    println!("{:?}", pizza_diameters.last());
    println!("third is {:?}", pizza_diameters.get(2));
}
