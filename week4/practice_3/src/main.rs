fn main() {
    let name1 = "Basit Boy";
    println!("My name is {} ", name1);

    // find and replace
    let name2 = name1.replace("Basit", "bob");
    println!("You can also call me {} ", name2);
    let faculty = "faculty of place ";

    // find and replace 
    let school = faculty.replace("faculty", "School");
    println!("I am a dude of {} ", school);
}
