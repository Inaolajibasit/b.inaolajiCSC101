fn main() {
    let b:(i32, bool, f64) = (100, true, 10.4);
    print(b);
}
// passing the tuple as a parameter
fn print(x:(i32, bool, f64)){
    println!("Inside Print Method");
    let (age, is_male, gpa) = x;
    println!("Age is{}, isMale? {}, Gpa is {}",age, is_male, gpa);
}
