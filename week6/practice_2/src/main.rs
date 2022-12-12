fn main() {
    //calling a function
    let name = "basot";
    let age = 3000;
    my_grade(name);
    show_my_age(age);
}

fn my_grade( namee:&str )  {
    // function body
    println!("GreeTING from {}",namee);
} 
fn show_my_age(age: i64)  {
    println!("basit is {} years old ",age );
}