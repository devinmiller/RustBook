fn main() {
    let mut s = String::from("Purge the xenos!  ");

    s.push_str("The Emporer protects!  ");

    println!("{}", s);

    let hello = String::from("Hello, ");
    let world = String::from("World!");

    let phrase = hello + &world;

    println!("{}", phrase);

    let s1 = String::from("For more than a hundred centuries ");
    let s2 = String::from("The Emperor has sat immobile ");
    let s3 = String::from("on the Golden Throne of Earth. ");

    let s = format!("It is the 41st Millennium. {}{}{}", s1, s2, s3);

    println!("{}", s);

    for c in "For the Emporer!".chars() {
        println!("{}", c);
    }
}   
