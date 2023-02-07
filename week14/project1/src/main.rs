use std::io::Read;
use std::io;

fn main() {
    println!("Welcome to GlobalCom!!!");
    println!("Please select the category you fall under");
    println!("1 for Administrator");
    println!("2 for Project Manager");
    println!("3 for Employee");
    println!("4 for Customer");

    let mut input = String::new();
    println!("Please enter the category you fall under");
    io::stdin().read_line(&mut input).expect("Failed");
    let input_num:i32 = input.trim().parse().expect("failed");

    if input_num == 1 {
        admin();
    }

    if input_num == 2 {
        proMan();
    }
    if input_num == 3 {
        emp();
    }
    if input_num == 4 {
        cust();
    }
}

fn admin(){
    println!("Welcome ADMINISTRATOR");
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("The Globacom Database Structure \n{}",content);
}

fn proMan(){
    println!("Welcome PROJECT MANAGER");
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("The Globacom Project Table Structure \n{}",content);
}

fn emp(){
    println!("Welcome EMPLOYEE");
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("The Globacom Staff Table Structure \n{}",content);
}

fn cust(){
    println!("Welcome CUSTOMER");
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("The Globacom Customer Table Structure \n{}",content);
}