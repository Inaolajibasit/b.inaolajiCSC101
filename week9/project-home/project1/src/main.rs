use std::io;
use std::io::Write;

fn intro_function(){
    println!("\nWelcome back to Ernst & Young Global Limited \n ");
    println!("Hello, Please select which code you belong to \n");
    println!("( 7 ) for code 7 ");
    println!("( 8 ) for code 9 ");
    println!("( 9 ) for code 9 ");

    let mut input1 = String::new();
    println!("Select");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 7 {
        println!("\nYou are in Code 7");
        code_7();
        
    }else if a == 8 {
        println!("\nYou are in Code 8");
        code_8();
        
    }else if a == 9{
        println!("\nYou are in Code 9");
        code_9();
    }else{
        println!("Chose again");
        intro_function();
    }
}
fn main() {
    intro_function();
}

fn code_7() {
    println!("Please Select Your Name : \n");
    println!("( 1 ) for Algbona Jullet ");
    println!("( 2 ) for Akpevwe Iloka ");

    let mut input1 = String::new();
    println!("Select");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        
        let name = "Algbona Jullet";
        let department = "Consulting";
        let qualifications = "B.Sc";
        let code = "7";
        let service = vec!["Analytics Consulting Service",
        "Customer Experience",
        "Cybersecurity, strategy, risk, compliance and resilience ",
        "Digital transformation",
        "Risk consulting services",
        "Supply chain and operations",
        "Technology transformation"];

        
        let mut file1 = std::fs::File::create("Algbona_Jullet.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Algbona_Jullet 🍾 ");
        
        
    }else if a == 2 {
        println!("Welcome Akpevwe Iloka ");

        let name = "Akpevwe Iloka";
        let department = "Assurance";
        let qualifications = "HND";
        let code = "7";
        let service = vec!["Audit services",
        "Climate change and sustainability services",
        "Financial accounting advisory services",
        "Forensic and integrity services ",
        "Private client audit experience",
        "Accounting Link ",
        "Assurance "];

        
        let mut file1 = std::fs::File::create("Akpevwe_Iloka.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Akpevwe_Iloka🍾");
        
    }else{
        println!("Chose again");
        code_7();
    }
}

fn code_8() {

    println!("Please Select Your Name : \n");
    println!("( 1 ) for Adamu Sagamu ");
    println!("( 2 ) for Gbenga Daniels ");

    let mut input1 = String::new();
    println!("Select");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        
        let name = "Adamu Sagamu";
        let department = "Tax";
        let qualifications = "B.Sc";
        let code = "8";
        let service = vec!["Tax planning ",
        "Tax function operations ",
        "Tax policy and controversy",
        "Global trade ",
        "Tax accounting",
        "Tax compliance",
        "Transaction tax"];

        
        let mut file1 = std::fs::File::create("Adamu_Sagamu.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Adamu Sagamu 🍾 ");
        
        
    }else if a == 2 {
        println!("Welcome Gbenga Daniels ");

        let name = "Gbenga Daniels";
        let department = "People and Workforce";
        let qualifications = "HND";
        let code = "8";
        let service = vec!["Change management and experience ",
        "HR transformation",
        "Integrated workforce mobility ",
        "Learning and development consulting  ",
        "Recognition and reward advisory ",
        "Workforce analytics ",
        "People and workforce "];

        
        let mut file1 = std::fs::File::create("Gbenga_Daniels.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Gbenga Daniels 🍾");
        
    }else{
        println!("Chose again");
        code_8();
    }
}

fn code_9() {

    println!("Please Select Your Name : \n");
    println!("( 1 ) for Ehis Ero ");
    println!("( 2 ) for Maria Akinsola ");

    let mut input1 = String::new();
    println!("Select");
    io::stdin().read_line(&mut input1).expect("Not a String");
    let a:i32 = input1.trim().parse().expect("not a number");

    if a == 1 {
        
        let name = "Ehis Ero ";
        let department = "Strategy";
        let qualifications = "M.Sc";
        let code = "9";
        let service = vec!["Strategy consulting ",
        "Corporate and growth strategy  ",
        "Transaction strategy and execution ",
        "Restructuring and turn-around strategy ",
        "Industry strategy ",
        "Digital business building ",
        "Commercial strategy"];

        
        let mut file1 = std::fs::File::create("Ehis_Ero.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Ehis Ero  🍾 ");
        
        
    }else if a == 2 {
        println!("Welcome Maria Akinsola ");

        let name = "Maria Akinsola";
        let department = "Transactions and corporate finance";
        let qualifications = "M.Sc.";
        let code = "9";
        let service = vec!["Corporate finance ",
        "Divestment and carve-outs",
        "Sustainability and ESG Services ",
        "M&A advisory   ",
        "M&A integration ",
        "M&A technology and tools  ",
        "M&A advanced analytics "];

        
        let mut file1 = std::fs::File::create("Maria_Akinsola.txt").expect("Failed");

        file1.write_all("Name: ".as_bytes()).expect("Failed");
        file1.write_all(name.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Department: ".as_bytes()).expect("Failed");
        file1.write_all(department.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Qualifications: ".as_bytes()).expect("Failed");
        file1.write_all(qualifications.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Code: ".as_bytes()).expect("Failed");
        file1.write_all(code.as_bytes()).expect("Failed");
        file1.write_all("\n".as_bytes()).expect("Failed");

        file1.write_all("Services: ".as_bytes()).expect("Failed");
        for i in service{
            file1.write_all("\n".as_bytes()).expect("Failed");
            file1.write_all("==> ".as_bytes()).expect("Failed");
            file1.write_all(i.as_bytes()).expect("Failed");
        }

        println!("Your file has been successfully created Maria Akinsola 🍾");
        
    }else{
        println!("Chose again");
        code_9();
    }
}