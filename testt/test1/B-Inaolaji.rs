use std::io;

fn main() {
    println!("Hello, This is the Student Council Voter system and ");
    println!("Faculty incentive calculator  ");
    println!("1 to vote ");
    println!("2 to find incentive ");

    let mut input1 = String::new();
    println!("What do you want to do?");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        let mut fig = 0;
        while fig < 15 {
            student_council_vote();
            fig+=1
        }
        
    }else if a == 2 {
        let mut figo = 0;
        while figo < 50{
            face_pub();
            figo+=1
        }
        
    }


}

fn face_pub() {

    println!("This program only runs 50 times ");
    let mut name = String::new();
    println!("Name of Faculty");
    io::stdin().read_line(&mut name).expect("Not a String");

    let mut input1 = String::new();
    println!("Number of Published Papers ??");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");
    

    if a >= 3 && a < 5 {
        println!("{}",name);
        println!("your incentive is N500,000 ");
    } else if a >= 5 && a < 10 {
        println!("your incentive is N800,000 ");
    }else if a >= 10 {
        println!("your incentive is N1,000,000 ");
    }else if a < 3 {
        println!("your incentive is N100,000 ");
    }
}

fn student_council_vote() {
    println!("This program only runs 15 times ");
    println!("");
    println!("Lets Vote!!!!!🥳");
    println!("Are you ELIGIBLE ??");
    println!("Must be above 100lv, have a CGPA of 4.0 Or higher, and must be a Class Rep");


    let mut input1 = String::new();
    println!("What level are you in ??");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:f32 = input1.trim().parse().expect("not a number");

    let mut input2 = String::new();
    println!("What is your CGPA ??");
    io::stdin().read_line(&mut input2).expect("Not a String");
    let b:f32 = input1.trim().parse().expect("not a number");

    let mut input3 = String::new();
    println!("Are you a class Rep");
    println!("0 of No");
    println!("1 of Yes");
    io::stdin().read_line(&mut input3).expect("Not a String");
    let c:f32 = input1.trim().parse().expect("not a number");

    if a > 100.0 && b > 3.9 && c > 0.9 {
        println!("YaY You Can vote 🍾🍾🍾");
        let mut name = String::new();
        println!("Your name");
        io::stdin().read_line(&mut name).expect("Not a String");

        let mut email = String::new();
        println!("Your email");
        io::stdin().read_line(&mut email).expect("Not a String");

        let mut department = String::new();
        println!("Your department");
        io::stdin().read_line(&mut department).expect("Not a String");

        let mut state = String::new();
        println!("Your State of origin");
        io::stdin().read_line(&mut state).expect("Not a String");


        println!("Your name {}",name);
        println!("Your email {}",email);
        println!("Your department {}",name);
        println!("Your State of origin {}",name);
        



        println!("YaY You have vote 🍾🍾🍾");
    }else {
        println!("You can not Vote ❌❌❌❌❌");
    }
}
