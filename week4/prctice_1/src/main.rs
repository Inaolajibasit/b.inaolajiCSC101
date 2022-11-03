fn main() {
    let name = "Basit Inaolaji";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Some where on Planet Earth ";

    println!("Name {} ", name);
    println!("University: {}, \n Address: {}", uni, addr);

    let department:&'static str = "Computer science";
    let school:&'static str = "School of science and Technology ";
    println!("Department: {}, \nSchool: {}", department, school);
}
