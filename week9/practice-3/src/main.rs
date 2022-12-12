use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Could not find file ");
    println!("file is removed");
}
