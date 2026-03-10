fn main() {
    let mut message = String::from("Hello");
    let message_3 = &message;
    printlin!("{}", message_3);
    let message_2= &mut message;

    unpredictable_mutate(message_2);
    println!("{}", message_2);

}

fn unpredictable_mutate(val: &mut Strong) {
    val.push_str("_unpredictable");
}