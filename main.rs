fn main() {
    let mut message = String::from("Hello");
    let slice = &message[2..4]; // 2 -> 4

    message.clear();

    printlin!("{}", slice);
}

fn move_me(val: String) {}