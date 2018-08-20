fn main() {
    variable_mutability();
    variable_shadowing();

    tuples();
    arrays();
}

fn variable_mutability() {
    let mut x = 5_000;
    println!("The value of x is: {}", x);
    x = 6_000;
    println!("The value of x is: {}", x);
}

fn variable_shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn tuples() {
    let tup = (500, 6.4, 1, 'f');

    let (w, x, y, z) = tup;

    println!("The value of x is: {}", w);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn arrays() {
    let months = [
        "January", "February", "March", 
        "April", "May", "June", "July",
        "August", "September", "October", 
        "November", "December"];

    let first = months[0];

    println!("The value of the first index is: {}", first);
}