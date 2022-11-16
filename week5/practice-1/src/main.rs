// rust progrem to output name and age

use std::io;

fn main() {
    println!("\n Student Info Management sysytem");

    //input name
    println!("\n Please Enter your name. ");        
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read Input");
    println!("Your name is: {}",name );

    // input age 
    println!("\n Enter your Asge");     
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed to read input ");
    let age:i32 = age.trim().parse().expect("input Not an integer ");
    println!("Your Age is  {}",age );

}
