use std::io::Write;

fn main() {
    let lager = vec!["|| 33 Export ||", "|| Desperados ||", "|| Goldberg ||", "|| Gulder ||", "|| Heineken ||", "|| Star ||\n"];
    let stout = vec!["|| Legend ||", "|| Turbo King ||", "|| Williams ||", "||  ||", "||  ||", "||  ||\n"];
    let nonAlcoholic = vec!["|| Maltina ||", "|| Amstel Malta ||", "|| Malta Gold ||", "|| Fayrouz ||", "||  ||", "||  ||\n"];

    let content = "Nigerian Breweries Plc \n";

    let mut file = std::fs::File::create("data.txt").expect("Failed");
    file.write_all(content.as_bytes()).expect("Failed");

    for i in lager{
        // println!("yoo {}", i);
        file.write_all("\n".as_bytes()).expect("Failed");
        file.write_all(i.as_bytes()).expect("Failed");
    }
    for i in stout{
        // println!("yoo {}", i);
        file.write_all(i.as_bytes()).expect("Failed");
    }
    for i in nonAlcoholic{
        // println!("yoo {}", i);
        file.write_all(i.as_bytes()).expect("Failed");
    }
}
