// rust program to find the roots of the equations 

use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("input A");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:f32 = input1.trim().parse().expect("not a number");

    println!("input B");
    io::stdin().read_line(&mut input2).expect("Not a String");
    let b:f32 = input2.trim().parse().expect("not a number");

    println!("input C");
    io::stdin().read_line(&mut input3).expect("Not a String");
    let c:f32 = input3.trim().parse().expect("not a number");

    let e:f32 = (b * b) - (4.0 * a * c);

    //the + or - part
    let p:f32 = -b + e.sqrt();
    // println!(" This is for the +  part: {}",p );

    let q:f32 = -b - e.sqrt();
    // println!(" This is for the - part: {}",q );

    // the final part 
    let f:f32 = p / (2.0 * a) ;
    println!(" This is for the + part: {}",f );

    let fo:f32 = q / (2.0 * a) ;
    println!(" This is for the - part: {}",fo );

    // println!(" This is e {}",p );
z
}
