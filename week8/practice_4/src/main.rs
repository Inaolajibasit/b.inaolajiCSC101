fn main() {
    // name vector 
    let name = vec!["Mary","sam","sally","greg","ade","mark","june","ife"];

    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];
    let loc = vec!["lag", "abj", "oyo", "kog", "clt", "cal", "tai", "jap"];

    println!("\nAge allocated :\n");

    for i in 0..age.len(){
        println!("{} is {} years old and lives in {} \n",name[i],age[i], loc[i] );
    }
}
