fn main() {
    let my_string = String::from("hello world");

    let word = first_world(&my_string[..]);

    println!("{}", word);    

    let my_string_literal = "hello world";

    println!("{}", word);

    let word = first_world(&my_string_literal[..]);

    println!("{}", word);

    let word = first_world(my_string_literal);
    
    println!("{}", word);

}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}