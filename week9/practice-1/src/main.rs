use std::io::Write;


fn main() {
    let announce = "Week-9 basit is a fine boy \n";
    let dept = " Omo You owe Basit Money";

    let mut file = std::fs::File::create("data.txt").expect("Create Failed");    
    file.write_all("Welcome to basit house\n".as_bytes()).expect("Failed to write");
    file.write_all(announce.as_bytes()).expect("Failed to write");
    file.write_all(dept.as_bytes()).expect("Failed to write" );
    println!("\nData written to file.");
}
