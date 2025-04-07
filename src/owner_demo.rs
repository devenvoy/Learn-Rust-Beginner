pub fn main() {
    let num = 25;
    let result = add(num);

    let name = String::from("Devansh");
    let name2 = gives_ownership();

    takes_ownership(name);
    // let s2 = name; // similar to this

    println!("Num is {num} and result is {result}");
    // println!("name is {name}")   // borrow of move value error
    println!("name is {name2}");

    let name2 = takes_and_gives_back(name2);

    println!("New Name is {name2}");

    let len = calculate_length(&name2);

    println!("The lenght of {name2} is {len}");

}

fn takes_ownership(s: String) {
    // s string come into scope
    println!("Inside takes ownership {s}");
    // s String goes out of scope and `drop` is called . the backing memory is freed.
}

fn gives_ownership() -> String {
    String::from("Devansh Amdavadwala")
    // some value created into scope
    // value returned and ownership moved to calling function
}

fn takes_and_gives_back(name: String) -> String {
    // name comes into scope
    name
    // name and returned and moves out to calling function
}

fn add(num: i32) -> i32 {
    // make copy
    // some integer come into scope
    num + 10
    // new integer gose out of scope nothing special happens
}

fn calculate_length(s:&String)->usize{
    // this is called borrowing which include reference variable   
    s.len()
}
