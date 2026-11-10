fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;

    let my_heap_value: String = String::from("Hello");
    let my_heap_reference: &String = &my_heap_value;

    println!("my_stack_value is {my_stack_value}");
    println!("my_integer_reference is {my_integer_reference}");
    println!("my_heap_value is {my_heap_value}");
    println!("my_heap_reference is {my_heap_reference}");
}
