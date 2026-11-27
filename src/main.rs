fn main() {
    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    let iterator = flavors.iter_mut();

    for flavor in iterator {
        (*flavor).push_str(" Ice Cream");
    }

    let mut school_grades = [85, 90, 72, 92];

    for grade in &mut school_grades {
        *grade -= 2;
    }

    println!("{school_grades:?}");
}
