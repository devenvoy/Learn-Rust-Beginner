pub fn main() {
    // let mut vec: Vec<i32> = Vec::new();
    let mut vec: Vec<i32> = vec![10, 20, 30];

    vec.push(40);

    // accessing element using index strating from 0
    // crash when invalid index
    let third = &vec[2];

    // if index found return its value or get default value
    // get return option enum with some carrying value if found
    let fourth = vec.get(3).unwrap_or(&0);
    // what unwarp_or do inside
    // let invalid = match vec.get(30){
    //     Some(x) => x,
    //     None => &-1
    // };
    let invalid = vec.get(30).unwrap_or(&0);

    println!("Value of {:?}", vec);
    println!("Value of third {third}");
    println!("Value of fourth {fourth}");
    println!("Value of invalid {invalid}");

    // iterate over vector
    for i in &vec {
        print!("{} ", i);
    }
    println!();
    // using ( * ) Dereference operator to access value instead of memory and changing value while getting mutable reference of each element
    for i in &mut vec {
        *i += 50;
    }

    println!("Value of {:?}", vec);
}
