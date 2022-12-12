fn main() {
    // make an empty vector city
    let mut city : Vec<String> = Vec::new();
    //print city vector
    println!("this is the number of elements in {} ",city.len());
    // push new element into the vector 
    let mut input1 = String::new();
    println!("enter how many cities you want");
    std::io::stdin().read_line(&mut input1).expect("Failed");
    let city_num : i32 = input1.trim().parse().expect("invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("enter city {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("failed");
        let new_city:String = input2.trim().parse().expect("Failed");
        city.push(new_city); 
    }
    println!("your Preferred cities are:\n");
    let mut count=1;
    // loop ti iterate elements in vector
    for i in city{
        // iterating through elements in vector
        println!("{} {}", count, i);
        count+=1;
    }
}
