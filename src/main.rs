fn main() {
    let mut car: String = String::from("Red");
    let ref1: &mut String = &mut car;
    ref1.push_str("and Silver");
    let ref2: &mut String = &mut car;
    println!("{ref2}");
}
