fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("The value of s is: {}", s);
    println!("The value of first word is: {}", word);

    let word = second_word(&s);

    println!("The value of second word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}
