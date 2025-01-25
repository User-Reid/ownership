fn main() {
    let person: String = String::from("Reid");

    drop(person);
    println!("{person}");
}
