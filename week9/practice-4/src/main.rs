use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let some_shit = "\nhey guys lets go rob a bank";
    println!("Hello, world"); 
    let mut file = OpenOptions::new()
       .append(true).open("data.txt").expect("cannot open file");
    file.write_all("\nHello class ".as_bytes()).expect("writes failed");
    file.write_all("\nThis is the appendage to the document. "
        .as_bytes()).expect("writes failed");
    file.write_all(some_shit.as_bytes()).expect("Failed");
    println!("file append successful");
}

