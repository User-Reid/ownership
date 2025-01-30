fn main() {
    let registrations: (bool, bool, bool) = (true, false, true);
    let first: bool = registrations.0;
    println!("{first}, {registrations:?}");

    let languages: (String, String) = (String::from("Rust"), String::from("Javascript"));
    let first: &String = &languages.0;

    println!("{first}, {languages:?}")
}
