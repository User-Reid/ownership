fn main() {
    let my_stack_value: i32 = 2;
    let my_intiger_reference: &i32 = &my_stack_value;
    println!("{}", *my_intiger_reference);
    println!("{my_intiger_reference}");

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;
    println!("{my_heap_reference}");

    let potato: i32 = 2;
    println!("{potato}");
}
