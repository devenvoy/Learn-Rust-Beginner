pub fn main() {
    let penny = Coin::Penny;

    println!("The value of {:?} is {}", penny, penny.cent_value());

    println!("The Optional ex: {}",opt_add(5,Some(25)));

    // match is exhaustive - it must need provide that all case will be handled 
    // if we want to handle only one or twoo case and skip other many cases 
    // we can store it in variable and use or ignore with (other or _) 

    let dice = 9;
    match  dice {
        3 => "nice try skip move",
        6 | 12 => "you got second dice roll",
        other => &format!("Move {other} steps")
    };

    // now suppose we want to handle only one case check and leave other unhandled
    // in that time we can use if let syntax , use condition and execute body if case matched 
    // also execute else if used , in case of condition not matched
    let config_max = Some(3_u8);
    if let Some(x) = config_max {
        println!("The Maximum configuration set to {}",x);
    }
    else{
        // optional in this we can handle all other case that do not match condition
    }
}


// Option - means nullable in kt 
// but in rust it is enum and has two value Some<t> and None

fn opt_add(num:i32,op : Option<i32>)->i32{
    match op {
        Some(x) => x + num,
        None => num
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

impl Coin {
    fn cent_value(&self) -> u8 {
        match &self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alaska) => {
                println!("Printed in Alaska");
                25
            }
            Coin::Quarter(value) => {
                println!("Printed in {value:?}");
                25
            }
        }
    }
}
