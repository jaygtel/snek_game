fn main() {
    c();
    d();
    f();
}

fn a() {
    println!("calling A");
    e();
}

fn b() {
    pintln!("calling B")
}

fn c() {
    println!("calling C")
}

fn d() {
    println!("calling D");
}

fn e() {
    println!("calling E");
}

fn f() {
    println!("calling F");
    b();
}

// xxd -g1 main