fn main() {
    let mut s = String::from("Purge the xenos!  ");

    s.push_str("The Emporer protects!  ");

    println!("{}", s);

    let hello = String::from("Hello, ");
    let world = String::from("World!");

    let phrase = hello + &world;

    println!("{}", phrase);
}   
