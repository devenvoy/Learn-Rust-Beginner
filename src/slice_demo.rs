pub fn slice_demo() {
    let s = String::from("Hello World");

    let res = find_first_word(&s);

    println!(
        "For String {s} the First word is \"{res}\" with lenght {}",
        res.len()
    );
}

fn find_first_word(input: &str) -> &str {
    let bytes = input.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[..index];
        }
    }
    input
}
