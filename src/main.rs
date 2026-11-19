mod inventory;
mod orders;

use fake::{Fake, Faker};

use inventory::{FLOOR_SPACE, MANAGER as INVENTORY_MANAGER, products::Item};
use orders::MANAGER as ORDER_MANAGER;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDER_MANAGER, FLOOR_SPACE,
    );

    let fake_item: Item = Faker.fake();
    println!("{:?}", fake_item);
}
