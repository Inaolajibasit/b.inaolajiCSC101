fn main() {
    // using vec::new();
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\n The length of the Vec::new() is: {}",v.len() );

    //using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing the size of vector
    println!("\n The length of the Vec::new() is: {}",v.len() );

    let v = vec!["bob", "hans","Grace", "Effiong", "Basil", "Kareem", "Susan"];
    println!("{}",v.len() );
}
