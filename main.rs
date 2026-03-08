fn main() {
/*
    Values stored directly in variables must have a size known at compile time
    (i.e., they implement `Sized`).
    Such as these:
*/

    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a, b);

/*
    Types that don't have a compile-time size (like `str`, `[T]`, or `dyn Trait`)
    must be used behind pointers.

    `String` itself is stored on the stack, but the text data is stored on the heap.

    Stack
    +--------------------------------+
    | ptr | len | capacity |         |  <- String (24 bytes)
    +--------------------------------+

    ptr       -> pointer to heap data
    len       -> number of bytes currently used
    capacity  -> number of bytes allocated

    Heap
    +----------------------+
    | H | e | l | l | o |  |
    +----------------------+

    `ptr` points to the start of the heap allocation where the UTF-8 characters are stored.

    It should also be noted that Rust does not allow indexing strings directly. This is because
    Rust strings are UTF-8 encoded, and characters may occupy multiple bytes. For example:

    let s = "é";

    é uses 2 bytes in UTF-8, so:
    [195, 169]

    This is why Rust does not allow: s[0]; and instead requires you to use, for example:

    s.chars()
    s.bytes()
    s.get(..)

    An example of a String whose struct is stored on the stack, while its text data is stored on
    the heap and referenced by a pointer, can be found below.
*/

    let message = String::from("Hello");
    println!("{}", message);
}

fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}