fn main() {
    let _float_num: f32 = 3.14;
    let _float_num_2: f64 = 3.2334327489;

    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);

    let (a,_b,_c) = tup;
    println!("{}", a);

    let x = [1,5,6,7];

    println!("{}", x[2]);

    let y = [2; 6]; // you get an array of 6 items but only 2s e.g: [2, 2, 2, 2, 2, 2]
    println!("{}", y[5]);
}