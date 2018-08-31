use std::io;
use std::io::Read;
use std::io::ErrorKind;

use std::fs;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Unable to read from file.")
    };

    match read_username_from_file_short() {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Unable to read from file.")
    };

    match read_username_from_file_really_short() {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Unable to read from file.")
    };

    match fs::read_to_string("hello.txt") {
        Ok(s) => println!("{}", s),
        Err(_) => println!("Unable to read from file.")
    };

    //let f = File::open("hello2.txt").unwrap();
    //let f = File::open("hello2.txt").expect("Failed to open hello2.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_really_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}