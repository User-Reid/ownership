fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str("Add sugar");
    meal
}

fn main() {
    let mut current_meal: String = String::new();
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    add_salt()
}
