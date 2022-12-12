fn main() {
    // making a mutable tuple
    let mut mountain_height = ("Everest", 8848, "Fish_tall", 2993);

    println!("\noriginal tuple = {:?} \n", mountain_height);

    // change the 4th amd 3rd element of a mutable tuple
    mountain_height.2 = "Nigga";
    mountain_height.3 = 8983;

    println!("Changed tuple = {:?} \n", mountain_height);
}
