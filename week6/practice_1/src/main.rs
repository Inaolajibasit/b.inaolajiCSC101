fn main (){
    let arr:[i32;4] = [10,20,30,40];
    println!("Array is {:?}",arr );
    println!("array size is {}",arr.len() );

    for val in arr.iter(){
        println!("basit Boy");
        println!("Value is :{}",val );
    }



    
}













/*
====1======
fn main() {
    //calling a function
    my_grade();
}

fn my_grade( )  {
    // function body
    println!("GreeTING from function my_grade");
} 

=====2======
fn checker()  {
    let mut input = String::new();
    println!("Enter a Character:");
    io::stdin().read_line(&mut input).expect("Failed");
    let ch:char = input.trim().parse().expect("Failed");

    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit ", ch);

    }else {
        println!("Character '{}' is not a digit ", ch);
    }
}

fn main() {
    println!("Welcome to my program BoZo");
    checker();
}

========3======
fn get_pi() -> f64{
    let a:f64 = 22.0;
    let b:f64 = 7.0;
    let c:f64 = a/b;
    return c;
}

=====4======
use std:;io;

fn add(a: i32,b: i32)  {
    let sum = a +b;

    println!("Sum of a and b{}", sum);
}

fn main() {
    let mut input1 = String::new();
    println!("Enter A");
    io::stdin().read_line(&mut input1).expect("Failed ");
    let a:i32 = input1.trim().parse().expect("Failed");

    let mut input2 = String::new();
    println!("Enter B");
    io::stdin().read_line(&mut input2).expect("Failed ");
    let b:i32 = input2.trim().parse().expect("Failed");

    add(a, b);

======5======
fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is: {}", num);
}

fn mutate_num_to_zero(mut param_num: i32){
    param_num = param_num*0;
    println!(" Param_num value is :{}",param_num);
}

======6======
fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is: {}", num);
}

fn mutate_num_to_zero(param_num: &mut i32){
    *param_num = *param_num*0; // the reference
    println!(" Param_num value is :{}",param_num);
}
======7======
fn main (){
    // array with data types 
    let arr1:[i32;4] = [10,20,30,40];
    println!("\nArray with data type");
    println!("Array is {:?}", arr1);
    println!("array size is :{}", arr1.len());

    // Array with out data types 
    let arr2 = [10.4, 20.7, 30.4, 40.9, 51.2, 72.2];
    println!("\nArray without data type");
    println!("Array is {:?}", arr2);
    println!("array size is :{}", arr2.len());

    // Array with default values that create and 
    // initializes all its elemnts with a default value of -1.
    let arr3:[i32;5] = [-1;5];
    println!("\nArray default values type");
    println!("Array is {:?}", arr3);
    println!("array size is :{}", arr3.len());
}

====8=====
let city_arr:[&str;5] = ["Abuja","Port-harcourt","Madiguri","Kano","Lagos"];
    println!("array is {:?}",city_arr );
    println!("array size is :{}",city_arr.len() );

    for index in 0..5{
        println!("City index {} is located in: {}",index, city_arr[index] );
    }


*/
