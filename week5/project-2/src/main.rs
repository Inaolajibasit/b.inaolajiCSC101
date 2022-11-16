// programm to determine the pay for employee

use std::io;

fn main() {
    let mut name = String::new();
    let mut exp = String::new();
    let mut age = String::new();
    let experienced = true;

    println!("Please input your name ");
    io::stdin().read_line(&mut name).expect("Wrong input");

    println!("Please input your Age \n Must be from 60 to 20 ");
    io::stdin().read_line(&mut age).expect("Wrong input");
    let age:i32 = age.trim().parse().expect("Wrong input");

    println!("Please input your Experience \n 'Yes' or 'no' ");
    io::stdin().read_line(&mut exp).expect("Wrong input");


    println!(" Your Name: {}",name );
    println!(" Your Age: {}",age );
    println!(" Your Experience: {}",exp );

    if age >= 40 && experienced  {
        println!(" Your Incentive: 1,560,000💰💰💰💰💰" );
    } else if experienced && age >= 30 && age <= 40 {
        println!(" Your Incentive: 1,480,000 💰💰💰" );
    } else if experienced && age >= 28  {
        println!(" Your Incentive: 1,300,000 💰💰" );
    } else if !experienced  {
        println!(" Your Incentive: 100,000 💰" );
    }

}
