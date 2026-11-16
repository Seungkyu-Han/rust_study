fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}

fn main() {
    let availability = is_item_in_stock(true, true);

    match availability {
        Option::Some(true) => println!("item is available"),
        Option::Some(false) => println!("Your item doesn't exist in out system"),
        Option::None => println!("i am none"),
    }
}
