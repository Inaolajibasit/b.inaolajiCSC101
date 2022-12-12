use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("doc.txt").unwrap();
    let mut content  = String::new();
    file.read_to_string(&mut content).unwrap();
    print!("okay content read all the files from doc.txt file \n{}", content);
}
