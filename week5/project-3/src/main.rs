// rust program that displays the following menu

use std::io;

fn main() {
    println!("FOOD MENU !!!!!!!");
    println!("Press 1 for Poundo yam/ Edinkaiko Soup: ====== price N3200 ");
    println!("Press 2 for Fried Rice & Chicken : ====== price N3000 ");
    println!("Press 3 for Amala & Ewedu Soup: ====== price N2500 ");
    println!("Press 4 for Eba & Egusi Soup: ====== price N2000 ");
    println!("Press 5 for White Rice & Stew: ====== price N2500 ");
    let mut price:f64 = 0.00;
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Please place your order \n Must be from 1 to 5 ");
    io::stdin().read_line(&mut input1).expect("Wrong input");
    let food:i32 = input1.trim().parse().expect("Wrong input");

    if food == 1 {
        println!("You ordered Poundo yam/ Edinkaiko Soup  ");
        println!("Price: 3200  ");
        let price = 3200.0;

        println!("How many do you want? ");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let amount:f32 = input2.trim().parse().expect("Wrong input");

        let total = price * amount;
        //println!("Your Total is {} Naira",total );

        if total > 10000.0 {
            let discount:f32 = (10.0 / 100.0) * total;
            println!("Your Total is {} with discount Naira",discount );
        } else{
            println!("Your Total is {}Naira",total );
        }
    }
    if food == 2 {
        println!("You ordered Fried Rice & Chicken  ");
        println!("Price: 3000  ");
        let price = 3000.0;

        println!("How many do you want? ");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let amount:f32 = input2.trim().parse().expect("Wrong input");

        let total = price * amount;
        //println!("Your Total is {} Naira",total );

        if total > 10000.0 {
            let discount:f32 = (10.0 / 100.0) * total;
            println!("Your Total is {} with discount Naira",discount );
        } else{
            println!("Your Total is {}Naira",total );
        }
    }
    if food == 3 {
        println!("You ordered Amala & Ewedu Soup ");
        println!("Price: 2500  ");
        let price = 2500.0;

        println!("How many do you want? ");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let amount:f32 = input2.trim().parse().expect("Wrong input");

        let total = price * amount;
        //println!("Your Total is {} Naira",total );

        if total > 10000.0 {
            let discount:f32 = (10.0 / 100.0) * total;
            println!("Your Total is {} with discount Naira",discount );
        } else{
            println!("Your Total is {}Naira",total );
        }
    }
    if food == 4 {
        println!("You ordered Eba & Egusi Soup ");
        println!("Price: 2000  ");
        let price = 2000.0;

        println!("How many do you want? ");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let amount:f32 = input2.trim().parse().expect("Wrong input");

        let total = price * amount;
        //println!("Your Total is {} Naira",total );

        if total > 10000.0 {
            let discount:f32 = (10.0 / 100.0) * total;
            println!("Your Total is {} with discount Naira",discount );
        } else{
            println!("Your Total is {}Naira",total );
        }
    }
    if food == 5 {
        println!("You ordered Eba & Egusi Soup ");
        println!("Price: 25d00  ");
        let price = 2500.0;

        println!("How many do you want? ");
        io::stdin().read_line(&mut input2).expect("Wrong input");
        let amount:f32 = input2.trim().parse().expect("Wrong input");

        let total = price * amount;
        // println!("Your Total is {} Naira",total );

        if total > 10000.0 {
            let discount:f32 = (10.0 / 100.0) * total;
            println!("Your Total is {} with discount Naira",discount );
        } else{
            println!("Your Total is {}Naira",total );
        }
    }


}
