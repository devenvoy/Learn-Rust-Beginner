use std::collections::HashMap;

pub fn main(){
    // not included in prelude need to import manually
    let mut scores = HashMap::new();


    // store data to hashamp 
    let blue_team = String::from("blue");
    let yellow_team = String::from("yellow");

    // override new value for this key , if not exist create new 
    scores.insert(blue_team, 20);
    scores.insert(yellow_team, 50);

    // add key and value if does not exists in hashmap
    scores.entry(String::from("red")).or_insert(0);


    // access value from hashmap
    // get return Option type if exists return Some(value) else None
    // copied get copy in heap instead of getting reference to original value 
    // unwrap_or will unwrap if 
    let blue_score = scores.get("blue").copied().unwrap_or(0);

    println!("Blue team score {}",blue_score);

    // hash map is also collection so we can iterate on each element 

    for (key,value) in scores{
        println!("{:?} => {:?}",key ,value);
    }
    


}