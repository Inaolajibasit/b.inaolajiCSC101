use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut student_name : Vec<String> = Vec::new();
    let mut student_mn : Vec<String> = Vec::new();
    let mut student_department : Vec<String> = Vec::new();
    let mut student_level : Vec<String> = Vec::new();
    

    let mut input1 = String::new();
    println!("How many student are applying ?");
    std::io::stdin().read_line(&mut input1).expect("Failed");
    let student_num : i32 = input1.trim().parse().expect("Failed");

    for persons in 0..student_num{

        let mut input2 = String::new();
        println!("Enter student {} Name ", persons+1);
        std::io::stdin().read_line(&mut input2).expect("Failed");
        let stu_name = input2.trim().parse().expect("Failed");
        student_name.push(stu_name);

        let mut input3 = String::new();
        println!("Enter student {} Matrix Number ", persons+1);
        std::io::stdin().read_line(&mut input3).expect("Failed");
        let stu_mn = input3.trim().parse().expect("Failed");
        student_mn.push(stu_mn);

        let mut input4 = String::new();
        println!("Enter student {} Department ", persons+1);
        std::io::stdin().read_line(&mut input4).expect("Failed");
        let stu_department = input4.trim().parse().expect("Failed");
        student_department.push(stu_department);

        let mut input5 = String::new();
        println!("Enter student {} level ", persons+1);
        std::io::stdin().read_line(&mut input5).expect("Failed");
        let stu_level = input5.trim().parse().expect("Failed");
        student_level.push(stu_level);
    }

    let mut file = std::fs::File::create("data.txt").expect("Failed");
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Failed");
    println!("Data written to file \n");

    println!("your Students are:\n");
    let mut count=1;
    // loop ti iterate elements in vector
    file.write_all(" Name:         ".as_bytes()).expect("Failed");
    for i in student_name{
        // iterating through elements in vector
        println!("( {} ) {}", count, i);
        count+=1;
        file.write_all(" | ".as_bytes()).expect("Failed");
        file.write_all(i.as_bytes()).expect("Failed");
    }
    file.write_all(" \n".as_bytes()).expect("Failed");
    file.write_all(" Matrix Number:".as_bytes()).expect("Failed");
    for i in student_mn{
        // iterating through elements in vector
        println!("( {} ) {}", count, i);
        count+=1;
        file.write_all(" | ".as_bytes()).expect("Failed");
        file.write_all(i.as_bytes()).expect("Failed");
    }
    file.write_all(" \n".as_bytes()).expect("Failed");
    file.write_all(" Department:   ".as_bytes()).expect("Failed");
    for i in student_department{
        // iterating through elements in vector
        println!("( {} ) {}", count, i);
        count+=1;
        file.write_all(" | ".as_bytes()).expect("Failed");
        file.write_all(i.as_bytes()).expect("Failed");
    }
    file.write_all(" \n".as_bytes()).expect("Failed");
    file.write_all(" Level:        ".as_bytes()).expect("Failed");
    for i in student_level{
        // iterating through elements in vector
        println!("( {} ) {}", count, i);
        count+=1;
        file.write_all(" | ".as_bytes()).expect("Failed");
        file.write_all(i.as_bytes()).expect("Failed");
    }
    file.write_all(" \n".as_bytes()).expect("Failed");

    

}