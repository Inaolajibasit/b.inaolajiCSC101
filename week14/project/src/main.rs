use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
} 
