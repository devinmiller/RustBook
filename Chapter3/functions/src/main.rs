fn main() {
    let w = {
        let v = 20;
        println!("The value of v is: {}", v);
        v + 20
    };

    println!("The value of w is: {}", w);

    let z = multiply(10, 4);

    println!("The value of z is: {}", z);
}

fn multiply(x: i32, y:i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x * y
}
