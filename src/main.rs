fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}")
}

fn main() {
    let burger: String = String::from("Burger");
    add_fries(burger);
}
