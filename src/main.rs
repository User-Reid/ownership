fn main() {
    let mut car: String = String::from("Red");
    let ref1: &mut String = &mut car;
    let ref2: &String = &car;
    println!("{ref1}, {ref2}")
}
