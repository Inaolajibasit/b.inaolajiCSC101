fn main() {
    let b:(i32, bool, f64) = (100, true, 10.4);
    print(b);
}
// passing the tuple as a parameter
fn print(x:(i32, bool, f64)){
    println!("Inside Print Method");
    println!("{:?}", x);
}
