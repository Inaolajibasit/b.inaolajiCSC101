fn main() {
    let a:i32 = 2; // bit 10
    let B:i32 = 3; // bit 11

    let mut result:i32;

    result = a & B;
    println!("(a & b) => {}", result);

    result = a | B;
    println!("(a | b) => {}", result);

    result = a ^ B;
    println!("(a ^ b) => {}", result);

    result = !B;
    println!("(!b) => {}", result);

    result = a << B;
    println!("(a << b) => {}", result);

    result = a >> B;
    println!("(a >> b) => {}", result);
}
