// this is a program to calculate the area of a triangel
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter fist edge of 📐");
    io::stdin().read_line(&mut input1).expect("not a string");
    let a:f32 = input1.trim().parse().expect("Not a number");


    println!("Enter second edge of 📐");
    io::stdin().read_line(&mut input2).expect("not a string");
    let b:f32 = input1.trim().parse().expect("Not a number");

    println!("Enter third edge of 📐");
    io::stdin().read_line(&mut input3).expect("not a string");
    let c:f32 = input1.trim().parse().expect("Not a number");

    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of a triangle: {}",area );
}
