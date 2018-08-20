fn main() {
    mutable_string();

    shallow_copy();
    deep_copy();
}

fn mutable_string() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn shallow_copy() {
    let s1 = String::from("hello, world");
    let s1_pointer =  format!("{:p}", s1.as_ptr());

    println!("the pointer for s1 is: {}", s1_pointer);

    let s2 = s1;
    let s2_pointer = format!("{:p}", s2.as_ptr());

    println!("the pointer for s2 is: {}", s2_pointer);

    assert_eq!(s1_pointer, s2_pointer);
}

fn deep_copy() {
    let s1 = String::from("hello, world");
    let s1_pointer =  format!("{:p}", s1.as_ptr());

    println!("the pointer for s1 is: {}", s1_pointer);

    let s2 = s1.clone();
    let s2_pointer = format!("{:p}", s2.as_ptr());

    println!("the pointer for s2 is: {}", s2_pointer);

    assert_ne!(s1_pointer, s2_pointer);
}