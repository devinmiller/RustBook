fn main() {
    calc_length_transfer();
    calc_length_borrow();

    mutable_reference();
}

fn calc_length_transfer() {
    let s1 = String::from("transfer, world");

    let (s2, len) = calculate_length_transfer(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length_transfer(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calc_length_borrow() {
    let s1 = String::from("borrow, world");

    let len = calculate_length_borrow(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("hello");

    update_reference(&mut s);

    println!("The value of s is: {}", s);
}

fn update_reference(s: &mut String) {
    s.push_str(", mutable references!")
}
