struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("Pointer Number One") };
    let d = CustomSmartPointer { data: String::from("Pointer Number Two") };

    println!("CustomSmartPointers created.");

    drop(c);
}
