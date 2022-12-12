use std::io;

fn main() {
    let v = vec!['c','o','m','p','u','t','e','r',];

    let mut input1 = String::new();

    println!("Enter an index value btw (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");

    // index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("Invalid Input");

    // getting value oat given index value
    let ch:char = v[index];

    println!("{} is the character for index [{}]\n",ch, index );

}
