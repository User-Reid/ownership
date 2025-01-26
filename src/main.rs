fn print_my_value(value: String) {
    println!("You value is {value}")
}

fn example(x: i32) {
    println!("{x}")
}

fn main() {
    let apples: String = String::from("Oranges");
    let x: i32 = 32;
    println!("{apples}");
    print_my_value(apples);
    example(x);
}
