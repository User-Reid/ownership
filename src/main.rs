fn bake_cake() -> String {
    let cake: String = String::from("Chocolate Mousse");
    return cake;
}

fn main() {
    let cake: String = bake_cake();
    println!("I now have a {cake} cake");
}
