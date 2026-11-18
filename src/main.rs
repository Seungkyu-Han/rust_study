mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favorite_catrgory = inventory::products::ProductCategory::Hammer;

    let tall_ladder = inventory::products::Item {
        name: String::from("Ladder"),
        category: favorite_catrgory,
        quantity: 100,
    };

    println!("{:?}", tall_ladder);
}
