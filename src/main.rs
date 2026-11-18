mod inventory;
mod orders;

use inventory::products::{self, ProductCategory};
use inventory::{FLOOR_SPACE, talk_to_manager};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_catrgory = ProductCategory::Hammer;
    println!("My favorite category of item is {favorite_catrgory:?}");

    let tall_ladder = products::Item {
        name: String::from("Ladder"),
        category: favorite_catrgory,
        quantity: 100,
    };

    println!("{:?}", tall_ladder);
}
