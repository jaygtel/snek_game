fn main() {
    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';

    println!("{}", custom_num); // 98000 - underscore is ignored and is just for easy reading

    // 0x denotes a hex number
    // this is calculated similar to the bin numbers, except using base16 instead of base2
    println!("{}", hex_num); // 0xfa;

    // 0b denotes a binary number
    // 0 0 1 0 1 0 1 1
    // starting right to left each number above is calculated as 2^x
    // i.e. 2^0, 2^1, 2^2, 2^3, 2^4, 2^5, 2^6, 2^7 and so on.
    println!("{}", bin_num); // 0b0010_1011;

    println!("{}", byte_num); // b'A';
}