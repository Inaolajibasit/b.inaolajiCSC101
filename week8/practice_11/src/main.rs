fn main() {
    // mutable array
    let mut color = ["red", "green","yellow", "white"];

    println!("\nOriginal array = {:?}", color);

    //mutable slice
    let sliced_colors = &mut color[1..3];

    println!("First slice = {:?}", sliced_colors);
    
    sliced_colors[0] = "purple";

    println!("Changed slice = {:?}", sliced_colors);
}
