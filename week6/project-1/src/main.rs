// program to perform differnt calculations 
use std::io;

fn main() {
    println!("What calculations do you want to perform");
    println!("1 to find the Area of Trapezium formula ");
    println!("2 to find the Area of the rhombus formula");
    println!("3 to find the Area of Parallelogram formula ");
    println!("4 to find the Area of Cube formula");
    println!("5 to find the Volume of Cylinder formula");

    let mut equation = String::new();
    io::stdin().read_line(&mut equation).expect("Failed");
    let equation_num:i32 = equation.trim().parse().expect("Failed");

    println!("You picked {} ", equation_num );
    
    if equation_num == 1 {
        trap()
    }
    if equation_num == 2 {
        rhom()
    }
    if equation_num == 3 {
        para()
    }
    if equation_num == 4 {
        cube()
    }
    if equation_num == 5 {
        trap()
    }

}


fn trap() {
    println!("Lets find the area of a Trapiziumm 😎");

    println!("Input height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed");
    let height_num:f32 = height.trim().parse().expect("Failed");
    println!("Input Base1");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed");
    let base1_num:f32 = base1.trim().parse().expect("Failed");
    println!("Input base2");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed");
    let base2_num:f32 = base2.trim().parse().expect("Failed");

    let area:f32 = height_num * 0.5 * ( base1_num + base2_num );
    println!("The Area of the Trapezium is :{}", area ); 
    println!("Thank You 😎😎😎");

}

fn rhom() {
    println!("Lets find the area of a Rhombus 😎");

    println!("Input diagonal 1");
    let mut diag1 = String::new();
    io::stdin().read_line(&mut diag1).expect("Failed");
    let diag1_num:f32 = diag1.trim().parse().expect("Failed");

    println!("Input diagonal 2");
    let mut diag2 = String::new();
    io::stdin().read_line(&mut diag2).expect("Failed");
    let diag2_num:f32 = diag2.trim().parse().expect("Failed");

    let area:f32 = 0.5 * ( diag2_num * diag1_num );
    println!("The Area of the Rhombus is :{}", area ); 
    println!("Thank You 😎😎😎");

}

fn para() {
    println!("Lets find the area of a Parallelogram 😎");

    println!("Input  Base");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed");
    let base_num:f32 = base.trim().parse().expect("Failed");

    println!("Input altitude");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed");
    let altitude_num:f32 = altitude.trim().parse().expect("Failed");

    let area:f32 = base_num * altitude_num;
    println!("The Area of the Parallelogram is :{}", area ); 
    println!("Thank You 😎😎😎");

}

fn cube() {
    println!("Lets find the area of a Cube 😎");

    println!("Input  length of Sides ");
    let mut len = String::new();
    io::stdin().read_line(&mut len).expect("Failed");
    let len_num:f32 = len.trim().parse().expect("Failed");

    let area:f32 = 6.0 * (len_num * len_num);
    println!("The Area of the cube is :{}", area ); 
    println!("Thank You 😎😎😎");

}