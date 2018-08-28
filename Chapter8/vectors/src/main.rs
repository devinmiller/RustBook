fn main() {
    let mut vec = vec![1, 2, 3];

    vec.push(4);
    vec.push(5);
    vec.push(6);

    get_element_index(&vec, 0);
    get_element_index(&vec, 4);

    get_element_method(&vec, 5);
    get_element_method(&vec, 6);

    iterate_elements(&vec);
    iterate_mutable_elements(&mut vec);
    iterate_elements(&vec);
}

fn get_element_index(v: &Vec<i32>, i: usize) {
    let value = &v[i];

    println!("The value of index {} is:  {}", i, value);
}

fn get_element_method(v: &Vec<i32>, i: usize) {
    match v.get(i) {
        Some(value) => {
            println!("The value of index {} is:  {}", i, value);
        },
        None => {
            println!("Could not find index: {}", i);
        }
    };
}

fn iterate_elements(v: &Vec<i32>) {
    println!("Iterating over elements!");
    for value in v {
        println!("The value is:  {}", value);
    }
}

fn iterate_mutable_elements(v: &mut Vec<i32>) {
    for value in v {
        *value += 2;
    }
}