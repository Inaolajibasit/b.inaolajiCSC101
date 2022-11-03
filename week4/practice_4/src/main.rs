fn main() {
    let fullname = "basit Benson Bob";
    let department = "computer Science";
    let uni = "Pan-Atlantic University";


    let mut school = "School of science".to_string();
    // push string
    school.push_str(" and Tomtom");

    println!("My name is: {} ", fullname);
    //check length
    println!("the length my fullname is: {}", fullname.len() );
    println!("I am a student of {} place",department );
    println!("{}",school );
    println!("{}",uni );
}
