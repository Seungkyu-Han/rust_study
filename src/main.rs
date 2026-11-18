mod inventory;
mod orders;

use inventory::MANAGER;
use inventory::products::{Item, ProductCategory};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor",
        MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_catrgory = ProductCategory::Hammer;

    let tall_ladder = Item {
        name: String::from("Ladder"),
        category: favorite_catrgory,
        quantity: 100,
    };

    println!("{:?}", tall_ladder);
}
