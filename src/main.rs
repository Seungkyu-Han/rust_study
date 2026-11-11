fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    print_length(&values);
}

fn print_length(reference: &[i32; 6]) {
    println!("{}", reference.len());
}
