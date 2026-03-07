fn main() {
    c();
    d();
    f();
}

fn a() {
    println!("calling A");
    e();
}

/*

If we change fn a() to be as follows, then we will create a stack overflow

fn a() {
    println!("calling A");
    d();
}

This is because:

a() calls d()
d() calls a()
a() calls d() again
d() calls a() again ... and so on endlessly

Each function call uses a stack frame (memory that stores the function’s local variables and return
address). Since the recursion never stops, the stack keeps growing until it runs out of memory.

When the stack is exhausted, the program panics in Rust with a stack overflow. In most systems,
this terminates the program immediately.

It should be noted that this is a runtime error, not a compiling error.  Rust will allow you to
compile this code, but when you try to run it, the program will crash due to a stack overflow.

*/

fn b() {
    println!("calling B");
}

fn c() {
    println!("calling C");
}

fn d() {
    println!("calling D");
    a();
}

fn e() {
    println!("calling E");
}

fn f() {
    println!("calling F");
    b();
}

// xxd -g1 main